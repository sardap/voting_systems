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
    borda_count::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        BordaCountCreateElection, BordaCountElection, BordaCountVote,
    },
    create_get_election_id_endpoint, create_post_election_id_new_vote_endpoint,
    create_post_endpoint, create_ranked_choice_vote_validation,
    db::DbPool,
    elections::CreateElection,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(length(min = 1, max = 100))]
    options: Vec<String>,
    require_token: bool,
}

impl Into<BordaCountCreateElection> for CreateElectionRequest {
    fn into(self) -> BordaCountCreateElection {
        BordaCountCreateElection {
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
struct CreateBordaCountVoteRequest {
    votes: Vec<usize>,
}

fn make_vote(request: &CreateBordaCountVoteRequest, user_id: &uuid::Uuid) -> BordaCountVote {
    BordaCountVote {
        created_by: user_id.clone(),
        votes: request.votes.clone(),
    }
}

create_ranked_choice_vote_validation!(&CreateBordaCountVoteRequest, &BordaCountElection);

create_post_election_id_new_vote_endpoint!(
    CreateBordaCountVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/borda_count")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
