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
    convert_into_uuid_or_fail, create_post_election_id_new_vote_endpoint, create_post_endpoint,
    cumulative::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        CumulativeCreateElection, CumulativeElection, CumulativeVote,
    },
    db::DbPool,
    elections::CreateElection,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(length(min = 2, max = 100))]
    options: Vec<String>,
    require_token: bool,
    #[validate(range(min = 1, max = 100))]
    max_votes: usize,
}

impl Into<CumulativeCreateElection> for CreateElectionRequest {
    fn into(self) -> CumulativeCreateElection {
        CumulativeCreateElection {
            election_base: CreateElection {
                title: self.title,
                requires_token: self.require_token,
            },
            options: self.options,
            max_votes: self.max_votes,
        }
    }
}

create_post_endpoint!(add_election, CreateElectionRequest);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicCumulativeElection {
    pub id: String,
    pub title: String,
    pub options: Vec<String>,
    pub require_token: bool,
    pub max_votes: usize,
}

#[get("/{election_id}")]
async fn get_election_id_endpoint(
    pool: actix_web::web::Data<crate::db::DbPool>,
    election_id: actix_web::web::Path<String>,
) -> actix_web::HttpResponse {
    let election_id = convert_into_uuid_or_fail!(election_id.as_str());

    let election = actix_web::web::block(move || {
        let mut conn = pool.get().unwrap();
        get_election(&mut conn, &election_id)
    })
    .await
    .unwrap();

    match election {
        Some(election) => actix_web::HttpResponse::Ok()
            .json(PublicCumulativeElection {
                id: election.election.id.to_string(),
                title: election.election.title,
                options: election.options,
                require_token: election.election.requires_token,
                max_votes: election.max_votes,
            })
            .into(),
        None => actix_web::HttpResponse::NotFound()
            .body("election doesn't exist")
            .into(),
    }
}

crate::create_get_election_id_get_result_endpoint!(get_election, get_votes, get_result);

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CumulativeVoteRequest {
    #[validate(length(min = 2, max = 100))]
    votes: Vec<usize>,
}

fn make_vote(request: &CumulativeVoteRequest, user_id: &uuid::Uuid) -> CumulativeVote {
    CumulativeVote {
        created_by: user_id.clone(),
        votes: request.votes.clone(),
    }
}

fn vote_validation(
    request: &CumulativeVoteRequest,
    election: &CumulativeElection,
) -> Result<(), crate::routes::api::common::NewVoteError> {
    use crate::routes::api::common::NewVoteError;
    if election.options.len() != request.votes.len() {
        return Err(NewVoteError::InvalidVoteCount);
    }

    if request.votes.iter().sum::<usize>() > election.max_votes {
        return Err(NewVoteError::InvalidVoteCount);
    }

    Ok(())
}

create_post_election_id_new_vote_endpoint!(
    CumulativeVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/cumulative")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
