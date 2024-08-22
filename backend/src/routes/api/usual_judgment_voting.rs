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
    usual_judgment::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        UsualJudgmentCreateElection, UsualJudgmentElection, UsualJudgmentVote,
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

impl Into<UsualJudgmentCreateElection> for CreateElectionRequest {
    fn into(self) -> UsualJudgmentCreateElection {
        UsualJudgmentCreateElection {
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

pub fn validate_usual_judgment_votes(votes: &Vec<usize>) -> Result<(), ValidationError> {
    if votes.iter().any(|i| *i > 7) {
        return Err(ValidationError::new(
            "invalid usual judgment must be between 0 and 7",
        ));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct UsualJudgmentVoteRequest {
    #[validate(length(min = 2, max = 100), custom = "validate_usual_judgment_votes")]
    votes: Vec<usize>,
}

fn make_vote(request: &UsualJudgmentVoteRequest, user_id: &uuid::Uuid) -> UsualJudgmentVote {
    UsualJudgmentVote {
        created_by: user_id.clone(),
        votes: request.votes.iter().map(|i| (*i as i32).into()).collect(),
    }
}

fn vote_validation(
    request: &UsualJudgmentVoteRequest,
    election: &UsualJudgmentElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    use crate::routes::api::common::NewVoteError;
    if election.options.len() != request.votes.len() {
        return Err(NewVoteError::InvalidVoteCount);
    }

    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    UsualJudgmentVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/usual_judgment")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
