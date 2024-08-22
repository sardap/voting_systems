use crate::schema::{
    anti_plurality_elections, anti_plurality_votes, approval_elections, approval_votes,
    borda_count_elections, borda_count_votes, condorcet_method_elections, condorcet_method_votes,
    cumulative_elections, cumulative_votes, elections, majority_judgment_elections,
    majority_judgment_votes, preferential_elections, preferential_votes, score_elections,
    score_votes, single_non_transferable_elections, single_non_transferable_votes,
    single_party_elections, single_party_votes, star_elections, star_votes, stv_elections,
    stv_votes, three_two_one_elections, three_two_one_votes, usual_judgment_elections,
    usual_judgment_votes, voting_tokens,
};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq, Insertable, Clone)]
#[diesel(table_name = elections)]
pub struct Election {
    pub id: uuid::Uuid,
    pub requires_token: bool,
    pub title: String,
    pub manage_token: uuid::Uuid,
    #[diesel(deserialize_as = std::time::SystemTime)]
    pub created_time: Option<std::time::SystemTime>,
    pub public: bool,
    pub voting_locked: bool,
}

impl Eq for Election {}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = voting_tokens)]
pub struct VotingToken {
    pub election_id: uuid::Uuid,
    pub token: uuid::Uuid,
}

macro_rules! create_baseline_election {
    ($name:ident, $table:expr) => {
        #[derive(
            serde::Serialize, serde::Deserialize, diesel::Queryable, Debug, diesel::Insertable,
        )]
        #[diesel(table_name = $table)]
        pub struct $name {
            pub election_id: uuid::Uuid,
            pub options: Vec<Option<String>>,
        }

        impl $name {
            pub fn new(election_id: &uuid::Uuid, options: Vec<String>) -> Self {
                Self {
                    election_id: election_id.clone(),
                    options: options.into_iter().map(|x| Some(x)).collect(),
                }
            }
        }
    };
}

create_baseline_election!(PreferentialElection, preferential_elections);
create_baseline_election!(BordaCountElection, borda_count_elections);
create_baseline_election!(UsualJudgmentElection, usual_judgment_elections);
create_baseline_election!(AntiPluralityElection, anti_plurality_elections);
create_baseline_election!(ApprovalElection, approval_elections);
create_baseline_election!(StarElection, star_elections);
create_baseline_election!(SinglePartyElection, single_party_elections);
create_baseline_election!(ThreeTwoOneElection, three_two_one_elections);
create_baseline_election!(CondorcetMethodElection, condorcet_method_elections);
create_baseline_election!(MajorityJudgmentElection, majority_judgment_elections);

macro_rules! create_baseline_vote {
    ($name:ident, $table:expr, $votes_type:ty) => {
        #[derive(
            serde::Serialize, serde::Deserialize, diesel::Queryable, Debug, diesel::Insertable,
        )]
        #[diesel(table_name = $table)]
        pub struct $name {
            pub id: uuid::Uuid,
            pub election_id: uuid::Uuid,
            pub created_by: uuid::Uuid,
            pub votes: Vec<Option<$votes_type>>,
        }
    };
}

create_baseline_vote!(PreferentialVote, preferential_votes, i32);
create_baseline_vote!(BordaCountVote, borda_count_votes, i32);
create_baseline_vote!(ApprovalVote, approval_votes, bool);
create_baseline_vote!(StarVote, star_votes, i32);
create_baseline_vote!(ThreeTwoOneVote, three_two_one_votes, i32);
create_baseline_vote!(CondorcetMethodVote, condorcet_method_votes, i32);
create_baseline_vote!(MajorityJudgmentVote, majority_judgment_votes, i32);
create_baseline_vote!(ScoreVote, score_votes, i32);
create_baseline_vote!(UsualJudgmentVote, usual_judgment_votes, i32);

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = score_elections)]
pub struct ScoreElection {
    pub election_id: uuid::Uuid,
    pub options: Vec<Option<String>>,
    pub max_score: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = stv_elections)]
pub struct StvElection {
    pub election_id: uuid::Uuid,
    pub options: Vec<Option<String>>,
    pub elected_count: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = stv_votes)]
pub struct StvVote {
    pub id: uuid::Uuid,
    pub election_id: uuid::Uuid,
    pub created_by: uuid::Uuid,
    pub votes: Vec<Option<i32>>,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = cumulative_elections)]
pub struct CumulativeElection {
    pub election_id: uuid::Uuid,
    pub options: Vec<Option<String>>,
    pub max_votes: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = cumulative_votes)]
pub struct CumulativeVote {
    pub id: uuid::Uuid,
    pub election_id: uuid::Uuid,
    pub created_by: uuid::Uuid,
    pub votes: Vec<Option<i32>>,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = anti_plurality_votes)]
pub struct AntiPluralityVote {
    pub id: uuid::Uuid,
    pub election_id: uuid::Uuid,
    pub created_by: uuid::Uuid,
    pub vote: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = single_party_votes)]
pub struct SinglePartyVote {
    pub id: uuid::Uuid,
    pub election_id: uuid::Uuid,
    pub created_by: uuid::Uuid,
    pub voted: bool,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = single_non_transferable_elections)]
pub struct SNTVElection {
    pub election_id: uuid::Uuid,
    pub options: Vec<Option<String>>,
    pub elected_count: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = single_non_transferable_votes)]
pub struct SNTVVote {
    pub id: uuid::Uuid,
    pub election_id: uuid::Uuid,
    pub created_by: uuid::Uuid,
    pub votes: Vec<Option<bool>>,
}
