use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult},
};

create_election!(ApprovalElection);

create_get_election!(crate::schema::approval_elections, ApprovalElection);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApprovalCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
}

create_add_election!(
    ApprovalCreateElection,
    crate::models::ApprovalElection,
    crate::schema::approval_elections
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApprovalVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<bool>,
}

impl From<crate::models::ApprovalVote> for ApprovalVote {
    fn from(v: crate::models::ApprovalVote) -> Self {
        Self {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|i| i.unwrap()).collect(),
        }
    }
}

impl ApprovalVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> crate::models::ApprovalVote {
        crate::models::ApprovalVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            votes: self.votes.clone().into_iter().map(Some).collect(),
        }
    }
}

create_get_votes!(
    crate::schema::approval_votes,
    crate::models::ApprovalVote,
    ApprovalVote
);

create_add_vote!(
    crate::schema::approval_votes,
    crate::models::ApprovalVote,
    ApprovalVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApprovalTally {
    pub option_index: usize,
    pub approval_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApprovalResult {
    pub options: Vec<String>,
    pub winner: usize,
    pub approve_tally: Vec<ApprovalTally>,
    pub vote_count: usize,
    pub votes: Vec<ApprovalVote>,
}

pub fn get_result(election: &ApprovalElection, votes: &[ApprovalVote]) -> ApprovalResult {
    let mut vote_tally = vec![0; election.options.len()];
    for vote in votes {
        for (option_index, v) in vote.votes.iter().enumerate() {
            if *v {
                vote_tally[option_index] += 1;
            }
        }
    }

    let mut approve_tally: Vec<ApprovalTally> = vote_tally
        .iter()
        .enumerate()
        .map(|(option_index, approval_count)| ApprovalTally {
            option_index,
            approval_count: *approval_count,
        })
        .collect();

    approve_tally.sort_by(|a, b| b.approval_count.cmp(&a.approval_count));

    ApprovalResult {
        options: election.options.clone(),
        winner: approve_tally[0].option_index,
        approve_tally,
        vote_count: votes.len(),
        votes: votes.to_vec(),
    }
}
