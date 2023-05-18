use actix_web::{patch, web, Scope};
use std::str::FromStr;
use validator::Validate;

use actix_web::{get, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    check_key, convert_into_uuid_or_fail,
    db::{DbConnection, DbPool},
    elections::{self, change_election_public, change_election_voting_locked},
    handle_check_error,
    routes::auth::{create_election_token, Auth},
};

mod anti_plurality_voting;
mod approval_voting;
mod borda_count_voting;
mod common;
mod condorcet_method_voting;
mod cumulative;
mod majority_judgment_voting;
mod preferential_voting;
mod score_voting;
mod single_non_transferable_voting;
mod single_party_voting;
mod single_transferable_vote;
mod star_voting;
mod three_two_one_vote;
mod usual_judgment_voting;

#[derive(Debug, Serialize, Deserialize)]
struct MakeElectionTokenResponse {
    token: String,
}

#[derive(Debug)]
pub enum GetTokenError {
    NoElection,
    ElectionDoesNotRequireToken,
    CheckError(crate::routes::auth::CheckError),
}

#[get("/{election_id}/get_token")]
async fn get_token(
    pool: web::Data<DbPool>,
    election_id: web::Path<String>,
    auth: web::Query<Auth>,
) -> HttpResponse {
    uuid::Uuid::from_str(election_id.as_str()).unwrap();
    let election_id = convert_into_uuid_or_fail!(election_id.as_str());
    let api_key = convert_into_uuid_or_fail!(auth.api_key.as_str());
    info!("Got new request to make a token for {}", election_id);

    let result = web::block(move || {
        let mut conn: DbConnection = pool.get().unwrap();

        check_key!(&mut conn, &election_id, &api_key, GetTokenError::CheckError);

        let election = match elections::get_election(&mut conn, &election_id) {
            Some(election) => election,
            None => return Err(GetTokenError::NoElection),
        };

        if !election.requires_token {
            return Err(GetTokenError::ElectionDoesNotRequireToken);
        }

        Ok(create_election_token(&mut conn, &election_id))
    })
    .await
    .unwrap();

    match result {
        Ok(token) => HttpResponse::Ok().json(MakeElectionTokenResponse {
            token: token.to_string(),
        }),
        Err(err) => match err {
            GetTokenError::NoElection => HttpResponse::NotFound().finish(),
            GetTokenError::ElectionDoesNotRequireToken => {
                HttpResponse::BadRequest().body("Election does not require token")
            }
            GetTokenError::CheckError(err) => {
                info!("Got bac request to get a token for {}", election_id);
                handle_check_error!(err)
            }
        },
    }
}

#[derive(Debug)]
pub enum GetExtraError {
    NoElection,
    CheckError(crate::routes::auth::CheckError),
}

#[get("/{election_id}/get_extra")]
async fn get_extra(
    pool: web::Data<DbPool>,
    election_id: web::Path<String>,
    auth: web::Query<Auth>,
) -> HttpResponse {
    uuid::Uuid::from_str(election_id.as_str()).unwrap();
    let election_id = convert_into_uuid_or_fail!(election_id.as_str());
    let api_key = convert_into_uuid_or_fail!(auth.api_key.as_str());
    info!("Got new request to get extra for {}", election_id);

    let result = web::block(move || {
        let mut conn: DbConnection = pool.get().unwrap();

        check_key!(&mut conn, &election_id, &api_key, GetExtraError::CheckError);

        let extra = match elections::get_extra_election_info(&mut conn, &election_id) {
            Some(election) => election,
            None => return Err(GetExtraError::NoElection),
        };

        Ok(extra)
    })
    .await
    .unwrap();

    match result {
        Ok(extra) => HttpResponse::Ok().json(extra),
        Err(err) => match err {
            GetExtraError::NoElection => HttpResponse::NotFound().finish(),
            GetExtraError::CheckError(err) => handle_check_error!(err),
        },
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ChangePublic {
    pub new_public: Option<bool>,
    pub new_voting_locked: Option<bool>,
}

#[derive(Debug)]
pub enum ChangeExtraError {
    NoElection,
    CheckError(crate::routes::auth::CheckError),
}

#[patch("/{election_id}/change_extra")]
async fn election_id_change_extra(
    pool: web::Data<DbPool>,
    election_id: web::Path<String>,
    request: actix_web_validator::Json<ChangePublic>,
    auth: web::Query<Auth>,
) -> HttpResponse {
    uuid::Uuid::from_str(election_id.as_str()).unwrap();
    let election_id = convert_into_uuid_or_fail!(election_id.as_str());
    let api_key = convert_into_uuid_or_fail!(auth.api_key.as_str());
    info!(
        "Got new request to make a election public for {}",
        election_id
    );

    let result = web::block(move || {
        let mut conn: DbConnection = pool.get().unwrap();

        info!("About to check api key");
        check_key!(
            &mut conn,
            &election_id,
            &api_key,
            ChangeExtraError::CheckError
        );
        info!("api key checked");

        match elections::get_election(&mut conn, &election_id) {
            Some(election) => election,
            None => return Err(ChangeExtraError::NoElection),
        };

        if let Some(new_public) = request.new_public.as_ref() {
            change_election_public(&mut conn, &election_id, *new_public);
        }
        if let Some(voting_locked) = request.new_voting_locked.as_ref() {
            change_election_voting_locked(&mut conn, &election_id, *voting_locked);
        }
        Ok(())
    })
    .await
    .unwrap();

    match result {
        Ok(_) => HttpResponse::Ok().json("Updated"),
        Err(err) => match err {
            ChangeExtraError::NoElection => HttpResponse::NotFound().finish(),
            ChangeExtraError::CheckError(err) => handle_check_error!(err),
        },
    }
}

pub fn routes() -> Scope {
    web::scope("/api/v1")
        .service(get_token)
        .service(get_extra)
        .service(election_id_change_extra)
        .service(preferential_voting::routes())
        .service(single_transferable_vote::routes())
        .service(borda_count_voting::routes())
        .service(approval_voting::routes())
        .service(star_voting::routes())
        .service(cumulative::routes())
        .service(anti_plurality_voting::routes())
        .service(single_party_voting::routes())
        .service(three_two_one_vote::routes())
        .service(condorcet_method_voting::routes())
        .service(majority_judgment_voting::routes())
        .service(score_voting::routes())
        .service(usual_judgment_voting::routes())
        .service(single_non_transferable_voting::routes())
}
