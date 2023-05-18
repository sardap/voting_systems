use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{
        self, tally_ranked_votes, CreateElection, CreateElectionResult, RankedChoiceVote,
        RankedChoiceVoteTally,
    },
    models,
};

create_election!(CondorcetMethodElection);

create_get_election!(
    crate::schema::condorcet_method_elections,
    CondorcetMethodElection
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CondorcetMethodCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
}

create_add_election!(
    CondorcetMethodCreateElection,
    models::CondorcetMethodElection,
    crate::schema::condorcet_method_elections
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CondorcetMethodVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<usize>,
}

impl RankedChoiceVote for CondorcetMethodVote {
    fn ranked_votes(&self) -> Vec<usize> {
        self.votes.clone()
    }
}

impl From<crate::models::CondorcetMethodVote> for CondorcetMethodVote {
    fn from(v: crate::models::CondorcetMethodVote) -> Self {
        Self {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|v| v.unwrap() as usize).collect(),
        }
    }
}

impl CondorcetMethodVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> crate::models::CondorcetMethodVote {
        crate::models::CondorcetMethodVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            votes: self
                .votes
                .clone()
                .into_iter()
                .map(|i| Some(i as i32))
                .collect(),
        }
    }
}

create_get_votes!(
    crate::schema::condorcet_method_votes,
    models::CondorcetMethodVote,
    CondorcetMethodVote
);

create_add_vote!(
    crate::schema::condorcet_method_votes,
    models::CondorcetMethodVote,
    CondorcetMethodVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchedPair {
    pub runner: usize,
    pub opponent: usize,
    pub votes_for_runner: usize,
    pub votes_for_opponent: usize,
    pub difference: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CondorcetMethodResult {
    pub options: Vec<String>,
    pub matchups: Vec<Vec<usize>>,
    pub condorcet_winner: Option<usize>,
    pub matched_pairs: Option<Vec<MatchedPair>>,
    pub locked_in_pairwise_victories: Option<Vec<Vec<usize>>>,
    pub matched_pair_winner: Option<usize>,
    pub last_resort_winner: Option<usize>,
    pub votes: Vec<RankedChoiceVoteTally>,
    pub vote_count: usize,
}

fn creates_cycle(graph: &Vec<Vec<usize>>, runner: usize, opponent: usize) -> bool {
    let mut visited_nodes: HashSet<usize> = HashSet::new();
    let mut stack: VecDeque<usize> = VecDeque::new();

    stack.push_back(opponent);

    while let Some(current_node) = stack.pop_front() {
        if current_node == runner {
            return true;
        }

        if !visited_nodes.contains(&current_node) {
            visited_nodes.insert(current_node);

            for &neighbor in &graph[current_node] {
                stack.push_back(neighbor);
            }
        }
    }

    false
}

fn lock_in_pairwise_victories(
    pairwise_comparisons: &Vec<MatchedPair>,
    num_candidates: usize,
) -> Vec<Vec<usize>> {
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); num_candidates];

    for pairwise_comparison in pairwise_comparisons {
        let runner = pairwise_comparison.runner;
        let opponent = pairwise_comparison.opponent;

        if !creates_cycle(&graph, runner, opponent) {
            graph[runner].push(opponent);
        }
    }

    graph
}

fn determine_winners(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let num_candidates = graph.len();
    let mut winners: Vec<usize> = Vec::new();

    for candidate in 0..num_candidates {
        let mut is_winner = true;

        for opponents in graph.iter() {
            if opponents.contains(&candidate) {
                is_winner = false;
                break;
            }
        }

        if is_winner {
            winners.push(candidate);
        }
    }

    winners
}

fn break_pairwise_tie(
    rng: &mut rand_pcg::Pcg64,
    tied: &[usize],
    votes: &[CondorcetMethodVote],
) -> usize {
    struct BordaCount {
        index: usize,
        score: usize,
    }

    if tied.len() == 1 {
        return tied[0];
    }

    // Break tie using Borda count
    let mut borda_scores = vec![0; votes[0].votes.len()];
    for vote in votes {
        for (index, &score) in vote.votes.iter().enumerate() {
            // 1 worth the most
            borda_scores[index] += votes[0].votes.len() - score;
        }
    }

    let borda_scores = borda_scores
        .iter()
        .enumerate()
        .map(|(index, score)| BordaCount {
            index,
            score: *score,
        })
        .collect::<Vec<BordaCount>>();

    let max_score: usize = borda_scores
        .iter()
        .max_by(|a, b| a.score.cmp(&b.score))
        .unwrap()
        .score;
    let mut matching_max_score = borda_scores
        .into_iter()
        .filter(|i| i.score == max_score)
        .collect::<Vec<BordaCount>>();
    matching_max_score.shuffle(rng);

    return matching_max_score[0].index;
}

pub fn get_result(
    election: &CondorcetMethodElection,
    votes: &[CondorcetMethodVote],
) -> CondorcetMethodResult {
    let votes_ranked_vote = tally_ranked_votes(votes);

    if votes.len() == 0 {
        return CondorcetMethodResult {
            options: election.options.clone(),
            matchups: vec![vec![0; election.options.len()]; election.options.len()],
            condorcet_winner: None,
            matched_pairs: None,
            locked_in_pairwise_victories: None,
            matched_pair_winner: None,
            last_resort_winner: None,
            votes: Vec::new(),
            vote_count: votes.len(),
        };
    }

    let mut rng: Pcg64 = Seeder::from(election.election.title.to_string()).make_rng();

    let mut matchups = vec![vec![0; election.options.len()]; election.options.len()];

    // Create matchup matrix
    for i in 0..matchups.len() {
        let row = &mut matchups[i];
        for j in 0..row.len() {
            let col = &mut row[j];
            for vote in votes {
                let vote = &vote.votes;
                if vote[i] < vote[j] {
                    *col += 1;
                }
            }
        }
    }

    // Look for Condorcet winner
    let mut condorcet_winner = None;
    for option_index in 0..election.options.len() {
        let mut condorcet_winner_found = true;
        for other_index in 0..matchups[option_index].len() {
            if option_index == other_index {
                continue;
            }

            if matchups[option_index][other_index] <= matchups[other_index][option_index] {
                condorcet_winner_found = false;
                break;
            }
        }

        if condorcet_winner_found {
            condorcet_winner = Some(option_index);
            break;
        }
    }

    if let Some(option_index) = condorcet_winner {
        return CondorcetMethodResult {
            options: election.options.clone(),
            matchups,
            condorcet_winner: Some(option_index),
            matched_pairs: None,
            locked_in_pairwise_victories: None,
            matched_pair_winner: None,
            last_resort_winner: None,
            votes: votes_ranked_vote,
            vote_count: votes.len(),
        };
    }

    // Not Condorcet winner using Ranked Pairs
    /*
      a. Start by creating a list of all pairwise comparisons between
      candidates (runner, opponent) sorted in descending
      order by the strength of their victory
      (the difference in votes between runner and opponent).
    */
    let mut ranked_pairs = Vec::new();
    for i in 0..matchups.len() {
        for j in 0..matchups.len() {
            if i == j {
                continue;
            }

            ranked_pairs.push(MatchedPair {
                runner: i,
                opponent: j,
                votes_for_runner: matchups[i][j],
                votes_for_opponent: matchups[j][i],
                difference: (matchups[i][j] as i64) - (matchups[j][i] as i64),
            });
        }
    }
    // Sort by strength of victory
    ranked_pairs.sort_by(|a, b| b.difference.cmp(&a.difference));
    // Lock in pairwise victories
    let locked_in_pairwise_victories =
        lock_in_pairwise_victories(&ranked_pairs, election.options.len());

    let winners = determine_winners(&locked_in_pairwise_victories);
    // pairwise winner found
    if winners.len() >= 1 {
        let winner = break_pairwise_tie(&mut rng, &winners, votes);

        return CondorcetMethodResult {
            options: election.options.clone(),
            matchups,
            condorcet_winner: None,
            matched_pairs: Some(ranked_pairs),
            locked_in_pairwise_victories: Some(locked_in_pairwise_victories),
            matched_pair_winner: Some(winner),
            last_resort_winner: None,
            votes: votes_ranked_vote,
            vote_count: votes.len(),
        };
    }

    // Okay this is completely fucked just use AEC style preference voting
    let pref_election = crate::preferential_voting::PreferentialElection {
        election: election.election.clone(),
        options: election.options.clone(),
    };
    let pref_votes: Vec<crate::preferential_voting::PreferentialVote> = votes
        .iter()
        .map(|v| crate::preferential_voting::PreferentialVote {
            created_by: v.created_by,
            votes: v.votes.clone(),
        })
        .collect();
    let pref_result = crate::preferential_voting::get_election_winner(&pref_election, &pref_votes);

    CondorcetMethodResult {
        options: election.options.clone(),
        matchups,
        condorcet_winner,
        locked_in_pairwise_victories: Some(locked_in_pairwise_victories),
        matched_pairs: Some(ranked_pairs),
        matched_pair_winner: None,
        last_resort_winner: Some(pref_result.winner),
        votes: votes_ranked_vote,
        vote_count: votes.len(),
    }
}
