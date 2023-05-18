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
    score::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        PublicScoreElection, ScoreCreateElection, ScoreElection, ScoreVote,
    },
};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(length(min = 2, max = 100))]
    options: Vec<String>,
    require_token: bool,
    #[validate(range(min = 2, max = 100))]
    max_score: usize,
}

impl Into<ScoreCreateElection> for CreateElectionRequest {
    fn into(self) -> ScoreCreateElection {
        ScoreCreateElection {
            election_base: CreateElection {
                title: self.title,
                requires_token: self.require_token,
            },
            options: self.options,
            max_score: self.max_score,
        }
    }
}

create_post_endpoint!(add_election, CreateElectionRequest);

create_get_election_id_endpoint!(get_election, PublicScoreElection);

crate::create_get_election_id_get_result_endpoint!(get_election, get_votes, get_result);

pub fn validate_score_votes(votes: &Vec<usize>) -> Result<(), ValidationError> {
    if votes.iter().any(|i| *i > 100) {
        return Err(ValidationError::new(
            "invalid three_two_one must be between 0 and 5",
        ));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ScoreVoteRequest {
    #[validate(length(min = 2, max = 100), custom = "validate_score_votes")]
    votes: Vec<usize>,
}

fn make_vote(request: &ScoreVoteRequest, user_id: &uuid::Uuid) -> ScoreVote {
    ScoreVote {
        created_by: user_id.clone(),
        votes: request.votes.iter().map(|i| *i).collect(),
    }
}

fn vote_validation(
    request: &ScoreVoteRequest,
    election: &ScoreElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    use crate::routes::api::common::NewVoteError;
    if election.options.len() != request.votes.len() {
        return Err(NewVoteError::InvalidVoteCount);
    }

    if request.votes.iter().any(|i| *i > election.max_score) {
        return Err(NewVoteError::InvalidVoteOption);
    }

    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    ScoreVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/score")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
