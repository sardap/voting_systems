use std::collections::HashSet;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use log::info;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use serde::{Deserialize, Serialize};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{
        self, tally_ranked_votes, CreateElection, CreateElectionResult, RankedChoiceVote,
        RankedChoiceVoteTally,
    },
    models,
};

create_election!(PreferentialElection);

impl Into<models::PreferentialElection> for PreferentialElection {
    fn into(self) -> models::PreferentialElection {
        models::PreferentialElection {
            election_id: self.election.id,
            options: self.options.into_iter().map(|i| Some(i)).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreferentialCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
}

create_add_election!(
    PreferentialCreateElection,
    models::PreferentialElection,
    crate::schema::preferential_elections
);

create_get_election!(crate::schema::preferential_elections, PreferentialElection);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreferentialVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<usize>,
}

impl PreferentialVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> models::PreferentialVote {
        models::PreferentialVote {
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

impl RankedChoiceVote<usize> for PreferentialVote {
    fn ranked_votes(&self) -> Vec<usize> {
        self.votes.clone()
    }
}

impl From<models::PreferentialVote> for PreferentialVote {
    fn from(v: models::PreferentialVote) -> Self {
        Self {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|i| i.unwrap() as usize).collect(),
        }
    }
}

create_get_votes!(
    crate::schema::preferential_votes,
    models::PreferentialVote,
    PreferentialVote
);

create_add_vote!(
    crate::schema::preferential_votes,
    models::PreferentialVote,
    PreferentialVote
);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CandidateVoteRoundResult {
    name: String,
    votes: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LogEntry {
    eliminated: Vec<usize>,
    votes: Vec<CandidateVoteRoundResult>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ElectionWinner {
    pub candidates: Vec<String>,
    pub log: Vec<LogEntry>,
    pub winner: usize,
    pub votes: Vec<RankedChoiceVoteTally<usize>>,
    pub vote_count: usize,
}

fn break_elimination_tie(
    rng: &mut rand_pcg::Pcg64,
    preference_tally: &Vec<Vec<usize>>,
    candidates: &[usize],
    pref_number: usize,
) -> usize {
    let lowest_value = candidates
        .iter()
        .map(|c| preference_tally[*c][pref_number])
        .min()
        .unwrap();

    let matching_lowest = candidates
        .iter()
        .map(|c| *c)
        .filter(|c| preference_tally[*c][pref_number] == lowest_value)
        .collect::<Vec<usize>>();

    info!(
        "Trying to break tie checking pref number {} between {:?} lowest value is {}",
        pref_number,
        candidates
            .iter()
            .map(|c| (*c, preference_tally[*c][pref_number]))
            .collect::<Vec<(usize, usize)>>(),
        lowest_value
    );

    if matching_lowest.len() == 1 {
        return *matching_lowest.first().unwrap();
    }

    if pref_number + 1 >= preference_tally[0].len() {
        return *matching_lowest.choose(rng).unwrap();
    }

    return break_elimination_tie(rng, preference_tally, &matching_lowest, pref_number + 1);
}

fn get_preference_tally(candidate_count: usize, votes: &[PreferentialVote]) -> Vec<Vec<usize>> {
    let mut preference_tally = Vec::new();
    for _ in 0..candidate_count {
        preference_tally.push(vec![0; candidate_count]);
    }

    for vote in votes {
        for (vote, pref_number) in vote.votes.iter().enumerate() {
            preference_tally[vote][*pref_number] += 1;
        }
    }

    preference_tally
}

pub fn get_election_winner(
    election: &PreferentialElection,
    votes: &[PreferentialVote],
) -> ElectionWinner {
    let mut rng: Pcg64 = Seeder::from(election.election.title.to_string()).make_rng();

    let preference_tally = get_preference_tally(election.options.len(), votes);

    let votes = votes.to_vec();
    // votes.shuffle(&mut rng);

    let candidates = election.options.clone();
    let target_count = votes.len() / 2;

    let mut log = Vec::new();

    let mut eliminated_candidates = HashSet::new();
    let mut vote_counts: Vec<Vec<usize>> = Vec::new();
    // Loop through each preference level
    loop {
        vote_counts.push(vec![0; candidates.len()]);
        let vote_count = vote_counts.last_mut().unwrap();
        for vote in &votes {
            let mut votes: Vec<_> = vote
                .votes
                .iter()
                .enumerate()
                .map(|(x, y)| (x, *y))
                .collect();

            votes.sort_by_key(|(_, pref)| *pref);

            for option in votes {
                if !eliminated_candidates.contains(&option.0) {
                    vote_count[option.0] += 1;
                    break;
                }
            }
        }

        let entry: Vec<CandidateVoteRoundResult> = candidates
            .iter()
            .enumerate()
            .map(|(i, name)| CandidateVoteRoundResult {
                name: name.clone(),
                votes: vote_count[i],
            })
            .collect();
        info!("Candidates this round {:?}", entry);
        log.push(LogEntry {
            eliminated: eliminated_candidates.iter().map(|i| *i).collect(),
            votes: entry,
        });
        if vote_count.iter().any(|&v| v > target_count)
            || (candidates.len() - eliminated_candidates.len()) <= 2
        {
            break;
        }

        // Eliminate the lowest candidate
        let lowest_index = vote_count
            .iter()
            .enumerate()
            .min_by_key(|(i, &v)| {
                if eliminated_candidates.contains(i) {
                    usize::MAX
                } else {
                    v
                }
            })
            .unwrap()
            .0;
        let lowest_not_eliminated_vote_count = vote_count[lowest_index];
        let remove_list = vote_count
            .iter()
            .enumerate()
            .filter(|(i, &v)| {
                !eliminated_candidates.contains(i) && v == lowest_not_eliminated_vote_count
            })
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        let remove_index = if remove_list.len() > 1 {
            info!("Tie between candidates {:?} with {:?} votes will remove one with lowest first round", remove_list, lowest_not_eliminated_vote_count);
            break_elimination_tie(&mut rng, &preference_tally, &remove_list, 0)
        } else {
            remove_list[0]
        };
        info!(
            "Removing candidate {:?} with {:?} votes",
            remove_index, lowest_not_eliminated_vote_count
        );
        eliminated_candidates.insert(remove_index);
    }

    let last_vote_count = vote_counts.last().unwrap();

    let mut winner_index = None;
    for i in 0..candidates.len() {
        if last_vote_count[i] > target_count {
            winner_index = Some(i);
            break;
        }
    }

    let winner_index = match winner_index {
        Some(i) => i,
        None => {
            let mut tied_candidates: Vec<usize> = candidates
                .iter()
                .enumerate()
                .filter(|(i, _)| !eliminated_candidates.contains(i))
                .map(|(i, _)| i)
                .collect();

            while tied_candidates.len() > 1 {
                let remove_index =
                    break_elimination_tie(&mut rng, &preference_tally, &tied_candidates, 0);
                tied_candidates.remove(
                    tied_candidates
                        .iter()
                        .position(|i| *i == remove_index)
                        .unwrap(),
                );
            }
            tied_candidates[0]
        }
    };

    info!("Winner(s) {:?}", candidates[winner_index]);

    ElectionWinner {
        winner: winner_index,
        log,
        votes: tally_ranked_votes(&votes),
        vote_count: votes.len(),
        candidates,
    }
}
