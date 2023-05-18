use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult},
};

create_election!(AntiPluralityElection);

create_get_election!(
    crate::schema::anti_plurality_elections,
    AntiPluralityElection
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AntiPluralityCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
}

create_add_election!(
    AntiPluralityCreateElection,
    crate::models::AntiPluralityElection,
    crate::schema::anti_plurality_elections
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AntiPluralityVote {
    pub created_by: uuid::Uuid,
    pub vote: usize,
}

impl From<crate::models::AntiPluralityVote> for AntiPluralityVote {
    fn from(v: crate::models::AntiPluralityVote) -> Self {
        Self {
            created_by: v.created_by,
            vote: v.vote as usize,
        }
    }
}

impl AntiPluralityVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> crate::models::AntiPluralityVote {
        crate::models::AntiPluralityVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            vote: self.vote as i32,
        }
    }
}

create_get_votes!(
    crate::schema::anti_plurality_votes,
    crate::models::AntiPluralityVote,
    AntiPluralityVote
);

create_add_vote!(
    crate::schema::anti_plurality_votes,
    crate::models::AntiPluralityVote,
    AntiPluralityVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AntiPluralityTally {
    pub option_index: usize,
    pub vote_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AntiPluralityResult {
    pub options: Vec<String>,
    pub votes_tally: Vec<AntiPluralityTally>,
    pub winner: usize,
    pub vote_count: usize,
    pub votes: Vec<AntiPluralityVote>,
}

pub fn get_result(
    election: &AntiPluralityElection,
    votes: &[AntiPluralityVote],
) -> AntiPluralityResult {
    let mut count_tally = vec![0; election.options.len()];
    for vote in votes {
        count_tally[vote.vote] += 1;
    }

    let mut count_tally: Vec<AntiPluralityTally> = count_tally
        .into_iter()
        .enumerate()
        .map(|(i, count)| AntiPluralityTally {
            option_index: i,
            vote_count: count,
        })
        .collect();
    count_tally.sort_by(|a, b| a.vote_count.cmp(&b.vote_count));

    AntiPluralityResult {
        options: election.options.clone(),
        winner: count_tally[0].option_index,
        votes_tally: count_tally,
        vote_count: votes.len(),
        votes: votes.clone().to_vec(),
    }
}
