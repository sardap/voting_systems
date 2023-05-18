use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult},
};

create_election!(StarElection);

create_get_election!(crate::schema::star_elections, StarElection);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
}

create_add_election!(
    StarCreateElection,
    crate::models::StarElection,
    crate::schema::star_elections
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<usize>,
}

impl From<crate::models::StarVote> for StarVote {
    fn from(v: crate::models::StarVote) -> Self {
        Self {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|i| i.unwrap() as usize).collect(),
        }
    }
}

impl StarVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> crate::models::StarVote {
        crate::models::StarVote {
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

create_get_votes!(crate::schema::star_votes, crate::models::StarVote, StarVote);

create_add_vote!(crate::schema::star_votes, crate::models::StarVote, StarVote);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarTally {
    pub option_index: usize,
    pub points_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarRunoffScore {
    pub option_index: usize,
    pub vote_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarResult {
    pub options: Vec<String>,
    pub points_tally: Vec<StarTally>,
    pub runoff: Vec<StarRunoffScore>,
    pub winner: usize,
    pub vote_count: usize,
    pub votes: Vec<StarVote>,
}

pub fn get_result(election: &StarElection, votes: &[StarVote]) -> StarResult {
    let mut points_tally = vec![0; election.options.len()];
    for vote in votes {
        for (i, points) in vote.votes.iter().enumerate() {
            points_tally[i] += *points as usize;
        }
    }

    let mut points_tally: Vec<StarTally> = points_tally
        .into_iter()
        .enumerate()
        .map(|(i, points)| StarTally {
            option_index: i,
            points_count: points,
        })
        .collect();
    points_tally.sort_by(|a, b| b.points_count.cmp(&a.points_count));

    // Run off
    let mut a = StarRunoffScore {
        option_index: points_tally[0].option_index,
        vote_count: 0,
    };
    let mut b = StarRunoffScore {
        option_index: points_tally[1].option_index,
        vote_count: 0,
    };
    for vote in votes {
        let a_votes = vote.votes[a.option_index];
        let b_votes = vote.votes[b.option_index];
        if a_votes == b_votes {
            continue;
        }

        if a_votes > b_votes {
            a.vote_count += 1;
        } else {
            b.vote_count += 1;
        }
    }

    let mut runoff = vec![a, b];
    runoff.sort_by(|a, b| b.vote_count.cmp(&a.vote_count));

    StarResult {
        options: election.options.clone(),
        points_tally,
        winner: runoff[0].option_index,
        runoff,
        vote_count: votes.len(),
        votes: votes.clone().to_vec(),
    }
}
