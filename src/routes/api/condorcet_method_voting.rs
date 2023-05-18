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
    condorcet_method::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        CondorcetMethodCreateElection, CondorcetMethodElection, CondorcetMethodVote,
    },
    create_get_election_id_endpoint, create_post_election_id_new_vote_endpoint,
    create_post_endpoint, create_ranked_choice_vote_validation,
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

impl Into<CondorcetMethodCreateElection> for CreateElectionRequest {
    fn into(self) -> CondorcetMethodCreateElection {
        CondorcetMethodCreateElection {
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
struct CreateCondorcetMethodVoteRequest {
    votes: Vec<usize>,
}

fn make_vote(
    request: &CreateCondorcetMethodVoteRequest,
    user_id: &uuid::Uuid,
) -> CondorcetMethodVote {
    CondorcetMethodVote {
        created_by: user_id.clone(),
        votes: request.votes.clone(),
    }
}

create_ranked_choice_vote_validation!(&CreateCondorcetMethodVoteRequest, &CondorcetMethodElection);

create_post_election_id_new_vote_endpoint!(
    CreateCondorcetMethodVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/condorcet_method")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
