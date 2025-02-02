use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult},
    models,
};

create_election!(BordaCountElection);

create_get_election!(crate::schema::borda_count_elections, BordaCountElection);

#[derive(Serialize, Deserialize, serde_valid::Validate, Debug, Clone)]
pub struct BordaCountCreateElection {
    pub election_base: CreateElection,
    #[validate(max_items = 100)]
    #[validate(custom = crate::elections::valid_election_option)]
    pub options: Vec<String>,
}

create_add_election!(
    BordaCountCreateElection,
    models::BordaCountElection,
    crate::schema::borda_count_elections
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BordaCountVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<usize>,
}

impl From<models::BordaCountVote> for BordaCountVote {
    fn from(v: models::BordaCountVote) -> Self {
        Self {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|i| i.unwrap() as usize).collect(),
        }
    }
}

impl BordaCountVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> models::BordaCountVote {
        models::BordaCountVote {
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
    crate::schema::borda_count_votes,
    models::BordaCountVote,
    BordaCountVote
);

create_add_vote!(
    crate::schema::borda_count_votes,
    models::BordaCountVote,
    BordaCountVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BordaCountTally {
    pub option_index: usize,
    pub vote_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BordaCountResult {
    pub options: Vec<String>,
    pub winner: usize,
    pub vote_tally: Vec<BordaCountTally>,
    pub votes: Vec<BordaCountVote>,
    pub vote_count: usize,
}

pub fn get_result(election: &BordaCountElection, votes: &[BordaCountVote]) -> BordaCountResult {
    let mut vote_tally = vec![0; election.options.len()];
    for vote in votes {
        for (option_index, v) in vote.votes.iter().enumerate() {
            vote_tally[option_index] += v;
        }
    }

    let mut vote_tally: Vec<BordaCountTally> = vote_tally
        .iter()
        .enumerate()
        .map(|(option_index, vote_count)| BordaCountTally {
            option_index: option_index,
            vote_count: *vote_count,
        })
        .collect();

    vote_tally.sort_by(|a, b| b.vote_count.cmp(&a.vote_count));

    BordaCountResult {
        options: election.options.clone(),
        winner: vote_tally.first().unwrap().option_index,
        vote_tally,
        votes: votes.to_vec(),
        vote_count: votes.len(),
    }
}
