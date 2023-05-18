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
    routes::auth::Auth,
    three_two_one::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        ThreeTwoOneCreateElection, ThreeTwoOneElection, ThreeTwoOneVote,
    },
};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(length(min = 3, max = 100))]
    options: Vec<String>,
    require_token: bool,
}

impl Into<ThreeTwoOneCreateElection> for CreateElectionRequest {
    fn into(self) -> ThreeTwoOneCreateElection {
        ThreeTwoOneCreateElection {
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

pub fn validate_three_two_one_votes(votes: &Vec<usize>) -> Result<(), ValidationError> {
    if votes.iter().any(|i| *i > 2) {
        return Err(ValidationError::new(
            "invalid three_two_one must be between 0 and 5",
        ));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ThreeTwoOneVoteRequest {
    #[validate(length(min = 3, max = 100), custom = "validate_three_two_one_votes")]
    votes: Vec<usize>,
}

fn make_vote(request: &ThreeTwoOneVoteRequest, user_id: &uuid::Uuid) -> ThreeTwoOneVote {
    ThreeTwoOneVote {
        created_by: user_id.clone(),
        votes: request.votes.iter().map(|i| (*i as i32).into()).collect(),
    }
}

fn vote_validation(
    request: &ThreeTwoOneVoteRequest,
    election: &ThreeTwoOneElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    use crate::routes::api::common::NewVoteError;
    if election.options.len() != request.votes.len() {
        return Err(NewVoteError::InvalidVoteCount);
    }

    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    ThreeTwoOneVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/three_two_one")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
