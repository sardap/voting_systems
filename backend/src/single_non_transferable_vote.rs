use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_vote, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult, PublicElection},
    models,
};

use voting_systems::single_non_transferable_vote::{SNTVElection, SNTVResult, SNTVVote};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVElectionModeled {
    pub election: models::Election,
    pub sntv_election: SNTVElection,
}

impl Into<PublicElection> for SNTVElectionModeled {
    fn into(self) -> PublicElection {
        PublicElection {
            id: self.election.id.to_string(),
            title: self.election.title.to_string(),
            options: self.sntv_election.options,
            require_token: self.election.requires_token,
        }
    }
}

pub fn get_election(c: &mut diesel::PgConnection, id: &uuid::Uuid) -> Option<SNTVElectionModeled> {
    let base_election = match elections::get_election(c, id) {
        Some(election) => election,
        None => return None,
    };

    use crate::schema::single_non_transferable_elections;
    let single_non_transferable_elections: crate::models::SNTVElection =
        match single_non_transferable_elections::table
            .filter(single_non_transferable_elections::election_id.eq(id))
            .first::<crate::models::SNTVElection>(c)
            .optional()
            .unwrap()
        {
            Some(e) => e,
            None => return None,
        };

    Some(SNTVElectionModeled {
        election: base_election,
        sntv_election: SNTVElection {
            options: single_non_transferable_elections
                .options
                .into_iter()
                .map(|i| i.unwrap())
                .collect(),
            elected_count: single_non_transferable_elections.elected_count as usize,
        },
    })
}

#[derive(Serialize, Deserialize, serde_valid::Validate, Debug, Clone)]
pub struct SNTVCreateElection {
    pub election_base: CreateElection,
    #[validate(max_items = 100)]
    #[validate(custom = crate::elections::valid_election_option)]
    pub options: Vec<String>,
    pub elected_count: usize,
}

pub fn add_election(c: &mut diesel::PgConnection, arg: SNTVCreateElection) -> CreateElectionResult {
    let result = crate::elections::add_election(
        c,
        &arg.election_base.title,
        arg.election_base.requires_token,
    );

    use crate::schema::single_non_transferable_elections;
    diesel::insert_into(single_non_transferable_elections::table)
        .values(models::SNTVElection {
            election_id: result.election_id,
            options: arg.options.into_iter().map(Some).collect(),
            elected_count: arg.elected_count as i32,
        })
        .execute(c)
        .unwrap();

    return result;
}

pub struct SNTVVoteModeled(pub SNTVVote);

impl From<crate::models::SNTVVote> for SNTVVote {
    fn from(v: crate::models::SNTVVote) -> Self {
        Self {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|i| i.unwrap()).collect(),
        }
    }
}

impl SNTVVoteModeled {
    fn make_model(&self, election_id: &uuid::Uuid) -> crate::models::SNTVVote {
        crate::models::SNTVVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.0.created_by,
            votes: self.0.votes.clone().into_iter().map(Some).collect(),
        }
    }
}

create_get_votes!(
    crate::schema::single_non_transferable_votes,
    crate::models::SNTVVote,
    SNTVVote
);

create_add_vote!(
    crate::schema::single_non_transferable_votes,
    crate::models::SNTVVote,
    SNTVVoteModeled
);

pub fn get_result(election: &SNTVElectionModeled, votes: &[SNTVVote]) -> SNTVResult {
    voting_systems::single_non_transferable_vote::get_result(&election.sntv_election, votes)
}
