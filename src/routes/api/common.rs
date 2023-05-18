use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! create_get_election_id_endpoint {
    ($get_election:ident, $response_ty:ty) => {
        #[get("/{election_id}")]
        async fn get_election_id_endpoint(
            pool: actix_web::web::Data<crate::db::DbPool>,
            election_id: actix_web::web::Path<String>,
        ) -> actix_web::HttpResponse {
            let election_id = $crate::convert_into_uuid_or_fail!(election_id.as_str());

            let election = actix_web::web::block(move || {
                let mut conn = pool.get().unwrap();
                $get_election(&mut conn, &election_id)
            })
            .await
            .unwrap();

            match election {
                Some(election) => {
                    let public_election: $response_ty = election.into();
                    actix_web::HttpResponse::Ok().json(public_election).into()
                }
                None => actix_web::HttpResponse::NotFound()
                    .body("election doesn't exist")
                    .into(),
            }
        }
    };
    ($get_election:ident) => {
        create_get_election_id_endpoint!($get_election, crate::elections::PublicElection);
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewElectionResponse {
    pub id: String,
    pub key: String,
}

#[macro_export]
macro_rules! create_post_endpoint {
    ($add_election:ident, $request_struct:ident) => {
        #[post("")]
        async fn post_endpoint(
            pool: actix_web::web::Data<crate::db::DbPool>,
            request: actix_web_validator::Json<$request_struct>,
            auth: actix_web::web::Query<Auth>,
        ) -> actix_web::HttpResponse {
            $crate::check_key!(auth.api_key);
            info!("Got new request to create a election {:?}", request.title);

            let result = actix_web::web::block(move || {
                let mut conn: diesel::r2d2::PooledConnection<
                    diesel::r2d2::ConnectionManager<diesel::PgConnection>,
                > = pool.get().unwrap();
                add_election(&mut conn, request.into_inner().into())
            })
            .await
            .unwrap();

            actix_web::HttpResponse::Ok()
                .json(crate::routes::api::common::NewElectionResponse {
                    id: result.election_id.to_string(),
                    key: result.manage_token.to_string(),
                })
                .into()
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct VoteTokenQuery {
    pub vote_token: Option<String>,
}

#[derive(Debug)]
pub enum NewVoteError {
    ElectionNotFound,
    InvalidToken,
    InvalidVoteCount,
    InvalidVoteOption,
    DuplicatedVote,
    AlreadyVoted,
}

#[macro_export]
macro_rules! create_ranked_choice_vote_validation {
    ($request_type:ty, $election_type:ty) => {
        fn vote_validation(
            request: $request_type,
            election: $election_type,
        ) -> Result<(), crate::routes::api::common::NewVoteError> {
            use crate::routes::api::common::NewVoteError;
            if election.options.len() != request.votes.len() {
                return Err(NewVoteError::InvalidVoteCount);
            }

            if request
                .votes
                .iter()
                .any(|vote| *vote >= election.options.len())
            {
                return Err(NewVoteError::InvalidVoteOption);
            }

            if (1..request.votes.len()).any(|i| request.votes[i..].contains(&request.votes[i - 1]))
            {
                return Err(NewVoteError::DuplicatedVote);
            }

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! create_post_election_id_new_vote_endpoint {
    ($request_type:ty, $get_election:ident, $validate_vote:ident, $make_vote:ident, $add_vote:ident) => {
        #[post("/{election_id}/new_vote")]
        async fn post_election_id_new_vote(
            pool: actix_web::web::Data<DbPool>,
            req: actix_web::HttpRequest,
            request: actix_web_validator::Json<$request_type>,
            election_id: actix_web::web::Path<String>,
            query: actix_web::web::Query<crate::routes::api::common::VoteTokenQuery>,
        ) -> actix_web::HttpResponse {
            let election_id = $crate::convert_into_uuid_or_fail!(election_id.as_str());
            let vote_token = match query.vote_token.clone() {
                Some(token) => Some($crate::convert_into_uuid_or_fail!(token.as_str())),
                None => None,
            };

            let user_id = crate::routes::auth::get_created_uuid(&req);

            use crate::routes::api::common::NewVoteError;

            let result = actix_web::web::block(move || {
                let mut conn = pool.get().unwrap();

                let election = match get_election(&mut conn, &election_id) {
                    Some(election) => election,
                    None => return Err(NewVoteError::ElectionNotFound),
                };

                if election.election.voting_locked {
                    return Err(NewVoteError::ElectionNotFound);
                }

                let vote_token = vote_token.as_ref();

                if election.election.requires_token
                    && (vote_token.is_none()
                        || !crate::routes::auth::is_vote_token_valid(
                            &mut conn,
                            &election_id,
                            vote_token.unwrap(),
                        ))
                {
                    log::info!("Invalid token given");
                    return Err(NewVoteError::InvalidToken);
                }

                if let Err(err) = $validate_vote(&request, &election) {
                    return Err(err);
                }

                match add_vote(&mut conn, &election_id, $make_vote(&request, &user_id)) {
                    Ok(_) => (),
                    Err(err) => match err {
                        AddVoteError::AlreadyVoted => return Err(NewVoteError::AlreadyVoted),
                    },
                }

                if let Some(vote_token) = vote_token {
                    crate::routes::auth::remove_vote_token(&mut conn, &election_id, vote_token);
                }

                Ok(())
            })
            .await
            .unwrap();

            match result {
                Ok(_) => HttpResponse::Ok().into(),
                Err(err) => match err {
                    NewVoteError::ElectionNotFound => HttpResponse::NotFound()
                        .body("election doesn't exist")
                        .into(),
                    NewVoteError::InvalidToken => {
                        HttpResponse::Unauthorized().body("invalid token").into()
                    }
                    NewVoteError::InvalidVoteCount => {
                        HttpResponse::BadRequest().body("invalid vote count").into()
                    }
                    NewVoteError::InvalidVoteOption => HttpResponse::BadRequest()
                        .body("invalid vote option")
                        .into(),
                    NewVoteError::DuplicatedVote => {
                        HttpResponse::BadRequest().body("duplicated vote").into()
                    }
                    NewVoteError::AlreadyVoted => {
                        HttpResponse::BadRequest().body("already voted").into()
                    }
                },
            }
        }
    };
}

#[derive(Deserialize)]
pub struct OptionalAuth {
    pub api_key: Option<String>,
}

#[macro_export]
macro_rules! create_get_election_id_get_result_endpoint {
    ($get_election:ident, $get_votes:ident, $get_result:ident) => {
        #[get("/{election_id}/get_result")]
        async fn get_election_id_get_result_endpoint(
            pool: actix_web::web::Data<DbPool>,
            election_id: actix_web::web::Path<String>,
            query: actix_web::web::Query<crate::routes::api::common::OptionalAuth>,
        ) -> actix_web::HttpResponse {
            let election_id = $crate::convert_into_uuid_or_fail!(election_id.as_str());
            let api_key = if let Some(api_key) = query.api_key.as_ref() {
                Some($crate::convert_into_uuid_or_fail!(api_key.as_str()))
            } else {
                None
            };

            let result = actix_web::web::block(move || {
                let mut conn = pool.get().unwrap();

                let election = $get_election(&mut conn, &election_id);

                if let Some(election) = election.as_ref() {
                    if !election.election.public {
                        if let Some(api_key) = api_key {
                            $crate::check_key!(&mut conn, &election_id, &api_key);
                        } else {
                            return Err($crate::routes::auth::CheckError::InvalidApiKey);
                        }
                    }
                }

                Ok((election, $get_votes(&mut conn, &election_id)))
            })
            .await
            .unwrap();

            if let Err(err) = result {
                $crate::handle_check_error!(err);
            }

            let (election, votes) = result.unwrap();

            if election.is_none() {
                return HttpResponse::NotFound()
                    .body("election doesn't exist")
                    .into();
            }

            let election = election.unwrap();

            let result = $get_result(&election, &votes);

            HttpResponse::Ok().json(result).into()
        }
    };
}
