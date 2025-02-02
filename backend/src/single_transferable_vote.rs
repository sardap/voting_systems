use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use log::info;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use serde::{Deserialize, Serialize};

use crate::{
    create_add_vote, create_get_votes,
    elections::{
        self, tally_ranked_votes, CreateElection, CreateElectionResult, RankedChoiceVote,
        RankedChoiceVoteTally,
    },
    models,
};

#[derive(Serialize, Deserialize, serde_valid::Validate, Debug, Clone)]
pub struct StvElection {
    pub election: models::Election,
    #[validate(max_items = 100)]
    #[validate(custom = crate::elections::valid_election_option)]
    pub options: Vec<String>,
    pub elected_count: usize,
}

impl Into<crate::elections::PublicElection> for StvElection {
    fn into(self) -> crate::elections::PublicElection {
        crate::elections::PublicElection {
            id: self.election.id.to_string(),
            title: self.election.title.to_string(),
            options: self.options,
            require_token: self.election.requires_token,
        }
    }
}

pub fn get_election(c: &mut diesel::PgConnection, election_id: &uuid::Uuid) -> Option<StvElection> {
    let base_election = match elections::get_election(c, election_id) {
        Some(election) => election,
        None => return None,
    };

    use crate::schema::stv_elections;
    let stv_election: models::StvElection = match stv_elections::table
        .filter(stv_elections::election_id.eq(election_id))
        .first::<models::StvElection>(c)
        .optional()
        .unwrap()
    {
        Some(election) => election,
        None => return None,
    };

    Some(StvElection {
        election: base_election,
        options: stv_election
            .options
            .into_iter()
            .map(|i| i.unwrap())
            .collect(),
        elected_count: stv_election.elected_count as usize,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateStvElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
    pub elected_count: usize,
}

pub fn add_election(c: &mut diesel::PgConnection, arg: CreateStvElection) -> CreateElectionResult {
    let result = crate::elections::add_election(
        c,
        &arg.election_base.title,
        arg.election_base.requires_token,
    );

    use crate::schema::stv_elections;
    diesel::insert_into(stv_elections::table)
        .values(models::StvElection {
            election_id: result.election_id,
            options: arg.options.into_iter().map(Some).collect(),
            elected_count: arg.elected_count as i32,
        })
        .execute(c)
        .unwrap();

    result
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Clone)]
pub struct StvVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<usize>,
}

impl RankedChoiceVote<usize> for StvVote {
    fn ranked_votes(&self) -> Vec<usize> {
        self.votes.clone()
    }
}

impl Eq for StvVote {}

impl From<models::StvVote> for StvVote {
    fn from(vote: models::StvVote) -> Self {
        Self {
            created_by: vote.created_by,
            votes: vote
                .votes
                .into_iter()
                .map(|i| i.unwrap() as usize)
                .collect(),
        }
    }
}

impl StvVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> models::StvVote {
        models::StvVote {
            id: uuid::Uuid::new_v4(),
            election_id: *election_id,
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

create_get_votes!(crate::schema::stv_votes, models::StvVote, StvVote);

create_add_vote!(crate::schema::stv_votes, models::StvVote, StvVote);

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StvRound {
    pub vote_counts: HashMap<usize, usize>,
    pub eliminated_candidates: HashSet<usize>,
    pub elected_candidates: HashSet<usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StvResult {
    pub candidates: Vec<String>,
    pub elected_candidates: Vec<usize>,
    pub votes: Vec<RankedChoiceVoteTally<usize>>,
    pub rounds: Vec<StvRound>,
    pub vote_count: usize,
}

fn break_elected_tie(
    rng: &mut rand_pcg::Pcg64,
    preference_tally: &HashMap<usize, Vec<usize>>,
    candidates: &[usize],
    pref_number: usize,
) -> usize {
    let highest_value = candidates
        .iter()
        .map(|c| preference_tally[c][pref_number])
        .max()
        .unwrap();

    let matching_highest = candidates
        .iter()
        .filter(|c| preference_tally[c][pref_number] == highest_value)
        .map(|c| *c)
        .collect::<Vec<usize>>();

    info!(
        "Trying to break elected tie checking pref number {} between {:?} highest value is {}",
        pref_number,
        candidates
            .iter()
            .map(|c| (*c, preference_tally[c][pref_number]))
            .collect::<Vec<(usize, usize)>>(),
        highest_value
    );

    if matching_highest.len() == 1 {
        return *matching_highest.first().unwrap();
    }

    if pref_number + 1 >= preference_tally[&0].len() {
        return *matching_highest.choose(rng).unwrap();
    }

    return break_elected_tie(rng, preference_tally, &matching_highest, pref_number + 1);
}

fn break_elimination_tie(
    rng: &mut rand_pcg::Pcg64,
    preference_tally: &HashMap<usize, Vec<usize>>,
    candidates: &[usize],
    pref_number: usize,
) -> usize {
    let lowest_value = candidates
        .iter()
        .map(|c| preference_tally[c][pref_number])
        .min()
        .unwrap();

    let matching_lowest = candidates
        .iter()
        .filter(|c| preference_tally[c][pref_number] == lowest_value)
        .map(|i| *i)
        .collect::<Vec<usize>>();

    info!(
        "Trying to break elimination tie checking pref number {} between {:?} lowest value is {}",
        pref_number,
        candidates
            .iter()
            .map(|i| (*i, preference_tally[i][pref_number]))
            .collect::<Vec<(usize, usize)>>(),
        lowest_value
    );

    if matching_lowest.len() == 1 {
        return *matching_lowest.first().unwrap();
    }

    if pref_number + 1 >= preference_tally[&0].len() {
        return *matching_lowest.choose(rng).unwrap();
    }

    return break_elimination_tie(rng, preference_tally, &matching_lowest, pref_number + 1);
}

fn get_preference_tally(votes: &[StvVote], options: &[String]) -> HashMap<usize, Vec<usize>> {
    let mut preference_tally: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..options.len() {
        preference_tally.insert(i, vec![0; options.len()]);
    }

    for vote in votes {
        for (round, vote) in vote.votes.iter().enumerate() {
            let option_votes = preference_tally.get_mut(vote).unwrap();
            let count = option_votes.get_mut(round).unwrap();
            *count += 1;
        }
    }

    preference_tally
}

pub fn get_result(
    election: &StvElection,
    votes: &[StvVote],
    starting_eliminated_candidates: &[usize],
) -> StvResult {
    let mut rng: Pcg64 = Seeder::from(election.election.title.clone()).make_rng();

    let quota = max(
        (votes.len() as f64 / election.elected_count as f64).floor() as usize,
        1,
    );
    let mut elected_candidates = HashSet::new();
    let mut eliminated_candidates: HashSet<usize> =
        starting_eliminated_candidates.iter().map(|i| *i).collect();
    let mut filtered_votes = HashSet::new();
    let mut rounds: Vec<StvRound> = Vec::new();
    let preference_tally = get_preference_tally(&votes, &election.options);

    while elected_candidates.len() < election.elected_count
        && votes.len() - filtered_votes.len() >= quota
    {
        let mut vote_counts: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, vote) in votes.iter().enumerate() {
            if filtered_votes.contains(&i) {
                continue;
            }
            let highest_valid_vote = vote
                .votes
                .iter()
                .find(|v| !elected_candidates.contains(*v) && !eliminated_candidates.contains(*v))
                .unwrap();
            let vote_count = match vote_counts.get_mut(&highest_valid_vote) {
                Some(v) => v,
                None => {
                    vote_counts.insert(*highest_valid_vote, Vec::new());
                    vote_counts.get_mut(&highest_valid_vote).unwrap()
                }
            };
            vote_count.push(i);
        }

        let quota_hitting_candidates = vote_counts
            .clone()
            .into_iter()
            .filter(|(_, v)| v.len() >= quota)
            .collect::<HashMap<usize, Vec<usize>>>();
        if quota_hitting_candidates.len() > 0 {
            let max_votes = quota_hitting_candidates
                .iter()
                .map(|(_, v)| v.len())
                .max()
                .unwrap();
            let mut candidates_with_most_votes = quota_hitting_candidates
                .iter()
                .filter(|(_, v)| v.len() == max_votes)
                .map(|(k, _)| *k)
                .collect::<Vec<usize>>();
            candidates_with_most_votes
                .sort_by(|a, b| election.options[*a].cmp(&election.options[*b]));

            let elected_candidate =
                break_elected_tie(&mut rng, &preference_tally, &candidates_with_most_votes, 0);

            info!(
                "quota hit, candidates {:?} max votes {} final selected {}:{}",
                quota_hitting_candidates.keys(),
                max_votes,
                elected_candidate,
                election.options[elected_candidate]
            );
            elected_candidates.insert(elected_candidate);

            let votes = vote_counts.get_mut(&elected_candidate).unwrap();

            votes.shuffle(&mut rng);

            for i in 0..quota {
                filtered_votes.insert(votes[i]);
            }
        } else {
            // Eliminate if no quota is hit
            let lowest_count = vote_counts
                .iter()
                .min_by_key(|(_, v)| v.len())
                .unwrap()
                .1
                .len();
            let mut lowest_matching_candidates = vote_counts
                .iter()
                .filter(|(_, v)| v.len() == lowest_count)
                .map(|(k, _)| *k)
                .collect::<Vec<usize>>();
            lowest_matching_candidates
                .sort_by(|a, b| election.options[*a].cmp(&election.options[*b]));

            info!(
                "No quota hit, lowest count is {} matching candidates {:?}",
                lowest_count, lowest_matching_candidates
            );
            let to_eliminate =
                break_elimination_tie(&mut rng, &preference_tally, &lowest_matching_candidates, 0);

            info!(
                "eliminating lowest {}:{}",
                election.options[to_eliminate],
                vote_counts[&to_eliminate].len()
            );
            eliminated_candidates.insert(to_eliminate);
        }

        info!(
            "Round {} vote count {:?}",
            rounds.len(),
            vote_counts
                .iter()
                .map(|(k, v)| (election.options[*k].clone(), v.len()))
                .collect::<HashMap<String, usize>>()
        );

        // Fill out vote_counts with empty vectors for eliminated candidates
        for i in 0..election.options.len() {
            if !vote_counts.contains_key(&i) {
                vote_counts.insert(i, Vec::new());
            }
        }

        rounds.push(StvRound {
            vote_counts: vote_counts.iter().map(|(k, v)| (*k, v.len())).collect(),
            eliminated_candidates: eliminated_candidates.clone(),
            elected_candidates: elected_candidates.clone(),
        });
    }

    let elected_candidates = elected_candidates.iter().map(|i| *i).collect();

    StvResult {
        candidates: election.options.clone(),
        elected_candidates,
        votes: tally_ranked_votes(votes),
        rounds,
        vote_count: votes.len(),
    }
}
