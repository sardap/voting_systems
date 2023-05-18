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
    create_post_endpoint, create_ranked_choice_vote_validation,
    db::DbPool,
    elections::CreateElection,
    preferential_voting::{
        add_election, add_vote, get_election, get_election_winner, get_votes, AddVoteError,
        PreferentialCreateElection, PreferentialElection, PreferentialVote,
    },
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

impl Into<PreferentialCreateElection> for CreateElectionRequest {
    fn into(self) -> PreferentialCreateElection {
        PreferentialCreateElection {
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

crate::create_get_election_id_get_result_endpoint!(get_election, get_votes, get_election_winner);

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreatePreferentialVoteRequest {
    votes: Vec<usize>,
}

fn make_vote(request: &CreatePreferentialVoteRequest, user_id: &uuid::Uuid) -> PreferentialVote {
    PreferentialVote {
        created_by: user_id.clone(),
        votes: request.votes.clone(),
    }
}

create_ranked_choice_vote_validation!(&CreatePreferentialVoteRequest, &PreferentialElection);

create_post_election_id_new_vote_endpoint!(
    CreatePreferentialVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicElection {
    pub id: String,
    pub title: String,
    pub options: Vec<String>,
    pub require_token: bool,
}

pub fn routes() -> Scope {
    web::scope("/preferential_voting")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
