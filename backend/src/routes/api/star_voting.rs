use std::str::FromStr;

use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Scope,
};
use log::info;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{
    create_get_election_id_endpoint, create_post_election_id_new_vote_endpoint,
    create_post_endpoint,
    db::DbPool,
    elections::CreateElection,
    star::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        StarCreateElection, StarElection, StarVote,
    },
};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(length(min = 2, max = 100))]
    options: Vec<String>,
    require_token: bool,
}

impl Into<StarCreateElection> for CreateElectionRequest {
    fn into(self) -> StarCreateElection {
        StarCreateElection {
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

crate::create_get_election_id_get_result_endpoint!(get_election, get_votes, get_result);

pub fn validate_star_votes(votes: &Vec<usize>) -> Result<(), ValidationError> {
    if votes.iter().any(|i| *i > 5) {
        return Err(ValidationError::new("invalid star must be between 0 and 5"));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct StarVoteRequest {
    #[validate(length(min = 2, max = 100), custom = "validate_star_votes")]
    votes: Vec<usize>,
}

fn make_vote(request: &StarVoteRequest, user_id: &uuid::Uuid) -> StarVote {
    StarVote {
        created_by: user_id.clone(),
        votes: request.votes.clone(),
    }
}

fn vote_validation(
    request: &StarVoteRequest,
    election: &StarElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    use crate::routes::api::common::NewVoteError;
    if election.options.len() != request.votes.len() {
        return Err(NewVoteError::InvalidVoteCount);
    }

    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    StarVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/star")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
