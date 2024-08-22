use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_vote, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult, PublicElection},
    models,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CumulativeElection {
    pub election: models::Election,
    pub options: Vec<String>,
    pub max_votes: usize,
}

impl Into<PublicElection> for CumulativeElection {
    fn into(self) -> PublicElection {
        PublicElection {
            id: self.election.id.to_string(),
            title: self.election.title.to_string(),
            options: self.options,
            require_token: self.election.requires_token,
        }
    }
}

pub fn get_election(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
) -> Option<CumulativeElection> {
    let base_election = match elections::get_election(c, election_id) {
        Some(election) => election,
        None => return None,
    };

    use crate::schema::cumulative_elections;
    let cumulative_election: models::CumulativeElection = match cumulative_elections::table
        .filter(cumulative_elections::election_id.eq(election_id))
        .first::<models::CumulativeElection>(c)
        .optional()
        .unwrap()
    {
        Some(election) => election,
        None => return None,
    };

    Some(CumulativeElection {
        election: base_election,
        options: cumulative_election
            .options
            .into_iter()
            .map(|i| i.unwrap())
            .collect(),
        max_votes: cumulative_election.max_votes as usize,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CumulativeCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
    pub max_votes: usize,
}

pub fn add_election(
    c: &mut diesel::PgConnection,
    arg: CumulativeCreateElection,
) -> CreateElectionResult {
    let result = crate::elections::add_election(
        c,
        &arg.election_base.title,
        arg.election_base.requires_token,
    );

    use crate::schema::cumulative_elections;
    diesel::insert_into(cumulative_elections::table)
        .values(models::CumulativeElection {
            election_id: result.election_id,
            options: arg.options.into_iter().map(Some).collect(),
            max_votes: arg.max_votes as i32,
        })
        .execute(c)
        .unwrap();

    return result;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CumulativeVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<usize>,
}

impl CumulativeVote {
    pub fn make_model(&self, election_id: &uuid::Uuid) -> models::CumulativeVote {
        models::CumulativeVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            votes: self.votes.iter().map(|i| Some(*i as i32)).collect(),
        }
    }
}

impl From<crate::models::CumulativeVote> for CumulativeVote {
    fn from(vote: crate::models::CumulativeVote) -> Self {
        CumulativeVote {
            created_by: vote.created_by,
            votes: vote
                .votes
                .into_iter()
                .map(|i| i.unwrap() as usize)
                .collect(),
        }
    }
}

create_get_votes!(
    crate::schema::cumulative_votes,
    crate::models::CumulativeVote,
    CumulativeVote
);

create_add_vote!(
    crate::schema::cumulative_votes,
    crate::models::CumulativeVote,
    CumulativeVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CumulativeTally {
    pub option_index: usize,
    pub vote_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CumulativeResult {
    pub options: Vec<String>,
    pub votes_tally: Vec<CumulativeTally>,
    pub winner: usize,
    pub vote_count: usize,
    pub votes: Vec<CumulativeVote>,
}

pub fn get_result(election: &CumulativeElection, votes: &[CumulativeVote]) -> CumulativeResult {
    let mut points_tally = vec![0; election.options.len()];
    for vote in votes {
        for (i, points) in vote.votes.iter().enumerate() {
            points_tally[i] += *points as usize;
        }
    }

    let mut points_tally: Vec<CumulativeTally> = points_tally
        .into_iter()
        .enumerate()
        .map(|(i, points)| CumulativeTally {
            option_index: i,
            vote_count: points,
        })
        .collect();
    points_tally.sort_by(|a, b| b.vote_count.cmp(&a.vote_count));

    CumulativeResult {
        options: election.options.clone(),
        winner: points_tally[0].option_index,
        votes_tally: points_tally,
        vote_count: votes.len(),
        votes: votes.to_vec(),
    }
}
