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
    majority_judgment::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        MajorityJudgmentCreateElection, MajorityJudgmentElection, MajorityJudgmentVote, Rating,
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

impl Into<MajorityJudgmentCreateElection> for CreateElectionRequest {
    fn into(self) -> MajorityJudgmentCreateElection {
        MajorityJudgmentCreateElection {
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

pub fn validate_majority_judgment_votes(votes: &Vec<usize>) -> Result<(), ValidationError> {
    if votes.iter().any(|i| *i > 5) {
        return Err(ValidationError::new(
            "invalid three_two_one must be between 0 and 5",
        ));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct MajorityJudgmentVoteRequest {
    #[validate(
        length(min = 2, max = 100),
        custom = "validate_majority_judgment_votes"
    )]
    votes: Vec<usize>,
}

fn make_vote(request: &MajorityJudgmentVoteRequest, user_id: &uuid::Uuid) -> MajorityJudgmentVote {
    MajorityJudgmentVote {
        created_by: user_id.clone(),
        votes: request.votes.iter().map(|i| (*i as i32).into()).collect(),
    }
}

fn vote_validation(
    request: &MajorityJudgmentVoteRequest,
    election: &MajorityJudgmentElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    use crate::routes::api::common::NewVoteError;
    if election.options.len() != request.votes.len() {
        return Err(NewVoteError::InvalidVoteCount);
    }

    if request.votes.iter().any(|i| {
        let very_good: usize = Rating::VeryGood.into();
        *i > very_good
    }) {
        return Err(NewVoteError::InvalidVoteOption);
    }

    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    MajorityJudgmentVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/majority_judgment")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
