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
    single_party::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        SinglePartyCreateElection, SinglePartyElection, SinglePartyVote,
    },
};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(length(min = 1, max = 1))]
    options: Vec<String>,
    require_token: bool,
}

impl Into<SinglePartyCreateElection> for CreateElectionRequest {
    fn into(self) -> SinglePartyCreateElection {
        SinglePartyCreateElection {
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

#[derive(Debug, Serialize, Deserialize, Validate)]
struct SinglePartyVoteRequest {
    voted: bool,
}

fn make_vote(request: &SinglePartyVoteRequest, user_id: &uuid::Uuid) -> SinglePartyVote {
    SinglePartyVote {
        created_by: user_id.clone(),
        voted: request.voted,
    }
}

fn vote_validation(
    _: &SinglePartyVoteRequest,
    _: &SinglePartyElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    SinglePartyVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/single_party")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
