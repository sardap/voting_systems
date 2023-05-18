use std::str::FromStr;

use actix_web::HttpRequest;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Auth {
    pub api_key: String,
}

lazy_static! {
    pub static ref API_KEY: uuid::Uuid =
        uuid::Uuid::from_str(std::env::var("VOTE_AUTH_STRING").unwrap().as_str()).unwrap();
}

pub fn sleep_for_between_x_and_y_ms(x: u64, y: u64) {
    use rand::Rng;
    use std::thread::sleep;
    use std::time::Duration;

    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(x..y);
    sleep(Duration::from_millis(sleep_time));
}

#[macro_export]
macro_rules! convert_into_uuid_or_fail {
    ($election_id:expr) => {
        match uuid::Uuid::from_str($election_id) {
            Ok(val) => val,
            Err(_) => {
                return HttpResponse::BadRequest().body("Invalid ID given").into();
            }
        }
    };
}

pub fn is_vote_token_valid(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
    token: &uuid::Uuid,
) -> bool {
    info!(
        "Checking if vote token {:?} is valid for {:?} in the db",
        token, election_id
    );
    use crate::schema::voting_tokens;
    voting_tokens::table
        .select(voting_tokens::token)
        .filter(voting_tokens::election_id.eq(election_id))
        .filter(voting_tokens::token.eq(token))
        .first::<uuid::Uuid>(c)
        .optional()
        .unwrap()
        .is_some()
}

pub fn remove_vote_token(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
    token: &uuid::Uuid,
) {
    use crate::schema::voting_tokens;
    let err = diesel::delete(voting_tokens::table)
        .filter(voting_tokens::election_id.eq(election_id))
        .filter(voting_tokens::token.eq(token))
        .execute(c);
    if let Err(err) = err {
        println!("Error removing vote token: {:?}", err);
    }
}

pub fn is_election_token_valid(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
    token: &uuid::Uuid,
) -> bool {
    info!(
        "Checking if election token {:?} is valid for {:?} in the db",
        token, election_id
    );
    use crate::schema::elections;
    elections::table
        .select(elections::manage_token)
        .filter(elections::id.eq(election_id))
        .filter(elections::manage_token.eq(token))
        .first::<uuid::Uuid>(c)
        .optional()
        .unwrap()
        .is_some()
}

#[derive(Debug)]
pub enum CheckError {
    InvalidApiKey,
}

#[macro_export]
macro_rules! check_key {
    ($api_key:expr) => {
        if $api_key != *crate::routes::auth::API_KEY.to_string() {
            crate::routes::auth::sleep_for_between_x_and_y_ms(1000, 5000);
            return HttpResponse::Unauthorized().into();
        }
    };
    ($connection:expr, $election_id:expr, $api_key:expr, $err_wrapper:expr) => {
        if !crate::routes::auth::is_election_token_valid($connection, $election_id, $api_key) {
            crate::routes::auth::sleep_for_between_x_and_y_ms(1000, 5000);
            return Err($err_wrapper(crate::routes::auth::CheckError::InvalidApiKey));
        }
    };
    ($connection:expr, $election_id:expr, $api_key:expr) => {
        if !crate::routes::auth::is_election_token_valid($connection, $election_id, $api_key) {
            crate::routes::auth::sleep_for_between_x_and_y_ms(1000, 5000);
            return Err(crate::routes::auth::CheckError::InvalidApiKey);
        }
    };
}

#[macro_export]
macro_rules! handle_check_error {
    ($err:expr) => {
        match $err {
            crate::routes::auth::CheckError::InvalidApiKey => {
                return HttpResponse::Unauthorized().into();
            }
        }
    };
}

pub fn create_election_token(c: &mut diesel::PgConnection, election_id: &uuid::Uuid) -> uuid::Uuid {
    use crate::schema::voting_tokens;
    let token = uuid::Uuid::new_v4();
    let insert_value = crate::models::VotingToken {
        election_id: election_id.clone(),
        token: token.clone(),
    };

    diesel::insert_into(voting_tokens::table)
        .values(insert_value)
        .execute(c)
        .unwrap();

    token
}

pub fn get_created_uuid(request: &HttpRequest) -> uuid::Uuid {
    let _cookies = request.cookies().unwrap();
    let user_id = match request.cookie("pref_election_uuid") {
        Some(val) => val.value().to_string(),
        None => uuid::Uuid::new_v4().to_string(),
    };

    uuid::Uuid::from_str(&user_id).unwrap_or_default()
}
