use std::str::FromStr;

use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Scope,
};
use log::info;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    approval::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        ApprovalCreateElection, ApprovalElection, ApprovalVote,
    },
    create_get_election_id_endpoint, create_get_election_id_get_result_endpoint,
    create_post_election_id_new_vote_endpoint, create_post_endpoint,
    db::DbPool,
    elections::CreateElection,
    routes::auth::Auth,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(length(min = 1, max = 100))]
    options: Vec<String>,
    require_token: bool,
}

impl Into<ApprovalCreateElection> for CreateElectionRequest {
    fn into(self) -> ApprovalCreateElection {
        ApprovalCreateElection {
            election_base: CreateElection {
                title: self.title,
                requires_token: self.require_token,
            },
            options: self.options,
        }
    }
}

create_post_endpoint!(add_election, CreateElectionRequest);

create_get_election_id_endpoint!(get_election);

create_get_election_id_get_result_endpoint!(get_election, get_votes, get_result);

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ApprovalVoteRequest {
    votes: Vec<bool>,
}

fn make_vote(request: &ApprovalVoteRequest, user_id: &uuid::Uuid) -> ApprovalVote {
    ApprovalVote {
        created_by: user_id.clone(),
        votes: request.votes.clone(),
    }
}

fn vote_validation(
    request: &ApprovalVoteRequest,
    election: &ApprovalElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    use crate::routes::api::common::NewVoteError;
    if election.options.len() != request.votes.len() {
        return Err(NewVoteError::InvalidVoteCount);
    }

    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    ApprovalVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/approval")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
