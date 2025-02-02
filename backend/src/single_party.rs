use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult},
    models,
};

create_election!(SinglePartyElection);

create_get_election!(crate::schema::single_party_elections, SinglePartyElection);

#[derive(Serialize, Deserialize, serde_valid::Validate, Debug, Clone)]
pub struct SinglePartyCreateElection {
    pub election_base: CreateElection,
    #[validate(max_items = 100)]
    #[validate(custom = crate::elections::valid_election_option)]
    pub options: Vec<String>,
}

create_add_election!(
    SinglePartyCreateElection,
    models::SinglePartyElection,
    crate::schema::single_party_elections
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SinglePartyVote {
    pub created_by: uuid::Uuid,
    pub voted: bool,
}

impl From<models::SinglePartyVote> for SinglePartyVote {
    fn from(v: models::SinglePartyVote) -> Self {
        Self {
            created_by: v.created_by,
            voted: v.voted,
        }
    }
}

impl SinglePartyVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> models::SinglePartyVote {
        models::SinglePartyVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            voted: self.voted,
        }
    }
}

create_get_votes!(
    crate::schema::single_party_votes,
    models::SinglePartyVote,
    SinglePartyVote
);

create_add_vote!(
    crate::schema::single_party_votes,
    models::SinglePartyVote,
    SinglePartyVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SinglePartyResult {
    pub options: Vec<String>,
    pub filled_votes: usize,
    pub blank_votes: usize,
    pub won: bool,
    pub vote_count: usize,
}

pub fn get_result(election: &SinglePartyElection, votes: &[SinglePartyVote]) -> SinglePartyResult {
    let valid_votes = votes.iter().filter(|i| i.voted).count();

    SinglePartyResult {
        options: election.options.clone(),
        filled_votes: valid_votes,
        blank_votes: votes.len() - valid_votes,
        won: valid_votes > votes.len() / 2,
        vote_count: votes.len(),
    }
}
