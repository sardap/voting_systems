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
    check_key, convert_into_uuid_or_fail, create_get_election_id_endpoint,
    create_post_election_id_new_vote_endpoint, create_post_endpoint,
    create_ranked_choice_vote_validation,
    db::DbPool,
    elections::CreateElection,
    handle_check_error,
    routes::auth::Auth,
    single_transferable_vote::{
        add_election, add_vote, get_election, get_result, get_votes, AddVoteError,
        CreateStvElection, StvElection, StvVote,
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
    elected_count: u32,
}

impl Into<CreateStvElection> for CreateElectionRequest {
    fn into(self) -> CreateStvElection {
        CreateStvElection {
            election_base: CreateElection {
                title: self.title,
                requires_token: self.require_token,
            },
            options: self.options,
            elected_count: self.elected_count as usize,
        }
    }
}

create_post_endpoint!(add_election, CreateElectionRequest);

create_get_election_id_endpoint!(get_election);

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetResultQuery {
    pub api_key: String,
    pub pre_eliminated_candidates: Option<String>,
}

#[get("/{election_id}/get_result")]
async fn get_election_id_get_result_endpoint(
    pool: web::Data<DbPool>,
    election_id: web::Path<String>,
    query: web::Query<GetResultQuery>,
) -> HttpResponse {
    {
        if query.pre_eliminated_candidates.is_some()
            && query
                .pre_eliminated_candidates
                .as_ref()
                .unwrap()
                .split(",")
                .any(|i| i.parse::<usize>().is_err())
        {
            return HttpResponse::BadRequest()
                .body("pre_eliminated_candidates must be a comma separated list of numbers")
                .into();
        }
    }

    let election_id = convert_into_uuid_or_fail!(election_id.as_str());
    let api_key = convert_into_uuid_or_fail!(query.api_key.as_str());

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        check_key!(&mut conn, &election_id, &api_key);

        Ok((
            get_election(&mut conn, &election_id),
            get_votes(&mut conn, &election_id),
        ))
    })
    .await
    .unwrap();

    if let Err(err) = result {
        handle_check_error!(err);
    }

    let (election, votes) = result.unwrap();

    if election.is_none() {
        return HttpResponse::NotFound()
            .body("election doesn't exist")
            .into();
    }

    let election = election.unwrap();

    let eliminated_candidates =
        if let Some(pre_eliminated_candidates) = query.pre_eliminated_candidates.as_ref() {
            pre_eliminated_candidates
                .split(",")
                .map(|i| i.parse::<usize>().unwrap())
                .filter(|i| *i < election.options.len())
                .map(|i| i)
                .collect::<Vec<usize>>()
        } else {
            vec![]
        };

    let result = get_result(&election, &votes, &eliminated_candidates);

    HttpResponse::Ok().json(result).into()
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreatePreferentialVoteRequest {
    votes: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct CreateStvElectionVoteRequest {
    votes: Vec<usize>,
}

fn make_vote(request: &CreateStvElectionVoteRequest, user_id: &uuid::Uuid) -> StvVote {
    StvVote {
        created_by: user_id.clone(),
        votes: request.votes.clone(),
    }
}

create_ranked_choice_vote_validation!(&CreateStvElectionVoteRequest, &StvElection);

create_post_election_id_new_vote_endpoint!(
    CreateStvElectionVoteRequest,
    get_election,
    vote_validation,
    make_vote,
    add_vote
);

pub fn routes() -> Scope {
    web::scope("/single_transferable_vote")
        .service(post_endpoint)
        .service(get_election_id_endpoint)
        .service(get_election_id_get_result_endpoint)
        .service(post_election_id_new_vote)
}
