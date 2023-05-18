use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_vote, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult, PublicElection},
    models,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVElection {
    pub election: models::Election,
    pub options: Vec<String>,
    pub elected_count: usize,
}

impl Into<PublicElection> for SNTVElection {
    fn into(self) -> PublicElection {
        PublicElection {
            id: self.election.id.to_string(),
            title: self.election.title.to_string(),
            options: self.options,
            require_token: self.election.requires_token,
        }
    }
}

pub fn get_election(c: &mut diesel::PgConnection, id: &uuid::Uuid) -> Option<SNTVElection> {
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

    Some(SNTVElection {
        election: base_election,
        options: single_non_transferable_elections
            .options
            .into_iter()
            .map(|i| i.unwrap())
            .collect(),
        elected_count: single_non_transferable_elections.elected_count as usize,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVCreateElection {
    pub election_base: CreateElection,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<bool>,
}

impl From<crate::models::SNTVVote> for SNTVVote {
    fn from(v: crate::models::SNTVVote) -> Self {
        Self {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|i| i.unwrap()).collect(),
        }
    }
}

impl SNTVVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> crate::models::SNTVVote {
        crate::models::SNTVVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            votes: self.votes.clone().into_iter().map(Some).collect(),
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
    SNTVVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVTally {
    pub option_index: usize,
    pub vote_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVResult {
    pub options: Vec<String>,
    pub winners: Vec<usize>,
    pub vote_tally: Vec<SNTVTally>,
    pub votes: Vec<SNTVVote>,
    pub vote_count: usize,
}

pub fn get_result(election: &SNTVElection, votes: &[SNTVVote]) -> SNTVResult {
    let mut vote_tally = vec![0; election.options.len()];
    for vote in votes {
        for (option_index, v) in vote.votes.iter().enumerate() {
            if *v {
                vote_tally[option_index] += 1;
            }
        }
    }

    let mut vote_tally: Vec<SNTVTally> = vote_tally
        .iter()
        .enumerate()
        .map(|(option_index, vote_count)| SNTVTally {
            option_index,
            vote_count: *vote_count,
        })
        .collect();

    vote_tally.sort_by(|a, b| b.vote_count.cmp(&a.vote_count));

    let winners: Vec<usize> = vote_tally.clone()[0..election.elected_count]
        .to_vec()
        .iter()
        .map(|i| i.option_index)
        .collect();

    SNTVResult {
        options: election.options.clone(),
        winners,
        vote_tally: vote_tally.clone(),
        votes: votes.clone().to_vec(),
        vote_count: votes.len(),
    }
}
