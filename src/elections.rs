use std::collections::HashMap;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateElection {
    pub title: String,
    pub requires_token: bool,
}

pub fn get_election(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
) -> Option<crate::models::Election> {
    use crate::schema::elections;
    elections::table
        .filter(elections::id.eq(election_id))
        .first::<crate::models::Election>(c)
        .optional()
        .unwrap()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateElectionResult {
    pub election_id: uuid::Uuid,
    pub manage_token: uuid::Uuid,
}

pub fn add_election(
    c: &mut diesel::PgConnection,
    title: &str,
    requires_token: bool,
) -> CreateElectionResult {
    use crate::schema::elections;

    let token = uuid::Uuid::new_v4();
    let id = uuid::Uuid::new_v4();

    diesel::insert_into(elections::table)
        .values(crate::models::Election {
            id,
            requires_token,
            title: title.to_string(),
            manage_token: token,
            created_time: None,
            public: false,
            voting_locked: false,
        })
        .execute(c)
        .unwrap();

    CreateElectionResult {
        election_id: id,
        manage_token: token,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicElection {
    pub id: String,
    pub title: String,
    pub options: Vec<String>,
    pub require_token: bool,
}

pub trait RankedChoiceVote {
    fn ranked_votes(&self) -> Vec<usize>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RankedChoiceVoteTally {
    pub votes: Vec<usize>,
    pub count: usize,
}

impl Eq for RankedChoiceVoteTally {}

pub fn tally_ranked_votes<T>(votes: &[T]) -> Vec<RankedChoiceVoteTally>
where
    T: RankedChoiceVote,
{
    let mut map: HashMap<Vec<usize>, usize> = HashMap::new();

    for vote in votes {
        let individual_votes = vote.ranked_votes();
        if !map.contains_key(&individual_votes) {
            map.insert(individual_votes.clone(), 0);
        }

        *map.get_mut(&individual_votes).unwrap() += 1;
    }

    let mut result: Vec<RankedChoiceVoteTally> = map
        .into_iter()
        .map(|(votes, count)| RankedChoiceVoteTally { votes, count })
        .collect();
    result.sort_by(|a, b| b.count.cmp(&a.count));

    result
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExtraElectionInfo {
    voting_lock: bool,
    locked: bool,
}

pub fn get_extra_election_info(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
) -> Option<ExtraElectionInfo> {
    let election = match get_election(c, election_id) {
        Some(election) => election,
        None => return None,
    };

    Some(ExtraElectionInfo {
        voting_lock: election.voting_locked,
        locked: election.public,
    })
}

pub fn change_election_public(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
    new_value: bool,
) {
    use crate::schema::elections;

    diesel::update(elections::table)
        .filter(elections::id.eq(election_id))
        .set(elections::public.eq(new_value))
        .execute(c)
        .unwrap();
}

pub fn change_election_voting_locked(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
    new_value: bool,
) {
    use crate::schema::elections;

    diesel::update(elections::table)
        .filter(elections::id.eq(election_id))
        .set(elections::voting_locked.eq(new_value))
        .execute(c)
        .unwrap();
}

#[macro_export]
macro_rules! create_election {
    ($name:ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        pub struct $name {
            pub election: crate::models::Election,
            pub options: Vec<String>,
        }

        impl $name {
            pub fn new(election: crate::models::Election, options: Vec<String>) -> Self {
                Self { election, options }
            }
        }

        impl Into<crate::elections::PublicElection> for $name {
            fn into(self) -> crate::elections::PublicElection {
                crate::elections::PublicElection {
                    id: self.election.id.to_string(),
                    title: self.election.title.to_string(),
                    options: self.options,
                    require_token: self.election.requires_token,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_get_election {
    ($options_path:path, $election_type:ty) => {
        pub fn get_election(
            c: &mut diesel::PgConnection,
            id: &uuid::Uuid,
        ) -> Option<$election_type> {
            let base_election = match elections::get_election(c, id) {
                Some(election) => election,
                None => return None,
            };

            let candidates: Option<Vec<Option<String>>>;
            {
                use $options_path::*;
                candidates = table
                    .filter(election_id.eq(id))
                    .select(options)
                    .first(c)
                    .optional()
                    .unwrap();
            }

            let options = match candidates {
                Some(candidates) => candidates,
                None => return None,
            };

            Some(<$election_type>::new(
                base_election,
                options.into_iter().map(|i| i.unwrap()).collect(),
            ))
        }
    };
}

#[macro_export]
macro_rules! create_add_election {
    ($create_arg_t:ty, $model:ty, $schema:path) => {
        pub fn add_election(
            c: &mut diesel::PgConnection,
            arg: $create_arg_t,
        ) -> CreateElectionResult {
            let result = crate::elections::add_election(
                c,
                &arg.election_base.title,
                arg.election_base.requires_token,
            );

            {
                use $schema::*;
                diesel::insert_into(table)
                    .values(<$model>::new(&result.election_id, arg.options))
                    .execute(c)
                    .unwrap();
            }

            return result;
        }
    };
}

#[macro_export]
macro_rules! create_get_votes {
    ($votes_path:path, $vote_model_type:ty, $vote_type:ty) => {
        pub fn get_votes(
            c: &mut diesel::PgConnection,
            target_election_id: &uuid::Uuid,
        ) -> Vec<$vote_type> {
            use $votes_path::*;
            let rows: Vec<$vote_model_type> = table
                .filter(election_id.eq(target_election_id))
                .get_results(c)
                .unwrap();

            rows.into_iter().map(<$vote_type>::from).collect()
        }
    };
}

#[macro_export]
macro_rules! create_add_vote {
    ($votes_path:path, $vote_model:ty, $vote_type:ty) => {
        pub enum AddVoteError {
            AlreadyVoted,
        }

        pub fn add_vote(
            c: &mut diesel::PgConnection,
            election_id: &uuid::Uuid,
            vote: $vote_type,
        ) -> Result<(), AddVoteError> {
            let insert_value = vote.make_model(election_id);

            {
                use $votes_path::*;
                if let Err(_) = diesel::insert_into(table).values(insert_value).execute(c) {
                    return Err(AddVoteError::AlreadyVoted);
                }
            }

            Ok(())
        }
    };
}
