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
    create_get_election_id_endpoint, create_post_election_id_new_vote_endpoint,
    create_post_endpoint,
    db::DbPool,
    elections::CreateElection,
    routes::auth::Auth,
    single_non_transferable_vote::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        SNTVCreateElection, SNTVElection, SNTVVote,
    },
};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(length(min = 1, max = 100))]
    options: Vec<String>,
    require_token: bool,
    #[validate(range(min = 1, max = 100))]
    elected_count: usize,
}

impl Into<SNTVCreateElection> for CreateElectionRequest {
    fn into(self) -> SNTVCreateElection {
        SNTVCreateElection {
            election_base: CreateElection {
                title: self.title,
                requires_token: self.require_token,
            },
            options: self.options,
            elected_count: self.elected_count,
        }
    }
}

create_post_endpoint!(add_election, CreateElectionRequest);

create_get_election_id_endpoint!(get_election);

crate::create_get_election_id_get_result_endpoint!(get_election, get_votes, get_result);

#[derive(Debug, Serialize, Deserialize, Validate)]
struct SNTVVoteRequest {
    votes: Vec<bool>,
}

fn make_vote(request: &SNTVVoteRequest, user_id: &uuid::Uuid) -> SNTVVote {
    SNTVVote {
        created_by: user_id.clone(),
        votes: request.votes.clone(),
    }
}

fn vote_validation(
    request: &SNTVVoteRequest,
    election: &SNTVElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    use crate::routes::api::common::NewVoteError;
    if election.options.len() != request.votes.len() {
        return Err(NewVoteError::InvalidVoteCount);
    }

    if request.votes.iter().filter(|x| **x).count() != 1 {
        return Err(NewVoteError::InvalidVoteCount);
    }

    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    SNTVVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/single_non_transferable")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
