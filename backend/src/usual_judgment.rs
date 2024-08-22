use core::panic;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use log::info;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use crate::{
    create_add_election, create_get_election,
    elections::{self, CreateElection, CreateElectionResult, PublicElection},
    models,
};

// HERE https://en.m.wikipedia.org/wiki/Usual_judgment

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsualJudgmentElection {
    pub election: models::Election,
    pub options: Vec<String>,
}

impl UsualJudgmentElection {
    pub fn new(election: models::Election, options: Vec<String>) -> Self {
        Self { election, options }
    }
}

impl Into<PublicElection> for UsualJudgmentElection {
    fn into(self) -> PublicElection {
        PublicElection {
            id: self.election.id.to_string(),
            title: self.election.title.to_string(),
            options: self.options,
            require_token: self.election.requires_token,
        }
    }
}

create_get_election!(
    crate::schema::usual_judgment_elections,
    UsualJudgmentElection
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsualJudgmentCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
}

create_add_election!(
    UsualJudgmentCreateElection,
    crate::models::UsualJudgmentElection,
    crate::schema::usual_judgment_elections
);

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Ord, PartialOrd, EnumIter,
)]
pub enum Grade {
    Bad,
    Inadequate,
    Passable,
    Fair,
    Good,
    VeryGood,
    Excellent,
}

impl Eq for Grade {}

impl Into<i32> for Grade {
    fn into(self) -> i32 {
        match self {
            Grade::Excellent => 6,
            Grade::VeryGood => 5,
            Grade::Good => 4,
            Grade::Fair => 3,
            Grade::Passable => 2,
            Grade::Inadequate => 1,
            Grade::Bad => 0,
        }
    }
}

impl Into<usize> for Grade {
    fn into(self) -> usize {
        match self {
            Grade::Excellent => 6,
            Grade::VeryGood => 5,
            Grade::Good => 4,
            Grade::Fair => 3,
            Grade::Passable => 2,
            Grade::Inadequate => 1,
            Grade::Bad => 0,
        }
    }
}

impl Into<f64> for Grade {
    fn into(self) -> f64 {
        match self {
            Grade::Excellent => 6.0,
            Grade::VeryGood => 5.0,
            Grade::Good => 4.0,
            Grade::Fair => 3.0,
            Grade::Passable => 2.0,
            Grade::Inadequate => 1.0,
            Grade::Bad => 0.0,
        }
    }
}

impl From<i32> for Grade {
    fn from(val: i32) -> Self {
        match val {
            6 => Grade::Excellent,
            5 => Grade::VeryGood,
            4 => Grade::Good,
            3 => Grade::Fair,
            2 => Grade::Passable,
            1 => Grade::Inadequate,
            0 => Grade::Bad,
            _ => panic!("Invalid rating"),
        }
    }
}

impl From<usize> for Grade {
    fn from(val: usize) -> Self {
        match val {
            6 => Grade::Excellent,
            5 => Grade::VeryGood,
            4 => Grade::Good,
            3 => Grade::Fair,
            2 => Grade::Passable,
            1 => Grade::Inadequate,
            0 => Grade::Bad,
            _ => panic!("Invalid rating"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsualJudgmentVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<Grade>,
}

pub fn get_votes(c: &mut diesel::PgConnection, election_id: &uuid::Uuid) -> Vec<UsualJudgmentVote> {
    use crate::schema::usual_judgment_votes;
    let votes: Vec<models::UsualJudgmentVote> = usual_judgment_votes::table
        .filter(usual_judgment_votes::election_id.eq(election_id))
        .get_results(c)
        .unwrap();

    votes
        .into_iter()
        .map(|i| UsualJudgmentVote {
            created_by: i.created_by,
            votes: i.votes.iter().map(|v| v.unwrap().into()).collect(),
        })
        .collect()
}

pub enum AddVoteError {
    AlreadyVoted,
}

pub fn add_vote(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
    vote: UsualJudgmentVote,
) -> Result<(), AddVoteError> {
    let insert_value = models::UsualJudgmentVote {
        id: uuid::Uuid::new_v4(),
        election_id: *election_id,
        created_by: vote.created_by,
        votes: vote.votes.into_iter().map(|i| Some(i.into())).collect(),
    };

    use crate::schema::usual_judgment_votes;
    if let Err(_) = diesel::insert_into(usual_judgment_votes::table)
        .values(insert_value)
        .execute(c)
    {
        return Err(AddVoteError::AlreadyVoted);
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsualJudgmentTally {
    pub option_index: usize,
    pub ratings: [i64; 7],
}

impl UsualJudgmentTally {
    fn majority_grade(&self) -> Grade {
        let vote_count = self.ratings.iter().sum::<i64>() as f64;
        let mut sum = 0.0;
        for (i, rating) in self.ratings.iter().enumerate() {
            sum += (*rating as f64) / vote_count;
            if sum >= 0.5 {
                return i.into();
            }
        }

        panic!("Impossible");
    }

    fn percent_for_grade(&self, grade: Grade) -> f64 {
        let total: i64 = self.ratings.iter().sum();
        let grade_index: usize = grade.into();
        let count = self.ratings[grade_index];

        count as f64 / total as f64
    }

    fn get_percent_above_grade(&self, target_grade: Grade) -> f64 {
        let mut sum = 0.0;
        for grade in Grade::iter() {
            if grade > target_grade {
                sum += self.percent_for_grade(grade);
            }
        }
        sum
    }

    fn get_percent_below_grade(&self, target_grade: Grade) -> f64 {
        let mut sum = 0.0;
        for grade in Grade::iter() {
            if grade < target_grade {
                sum += self.percent_for_grade(grade);
            }
        }
        sum
    }

    fn score_n(&self, n: f64) -> f64 {
        let majority_grade = self.majority_grade();
        let a: f64 = majority_grade.into();
        let p: f64 = self.get_percent_above_grade(majority_grade).powf(n);
        let q: f64 = self.get_percent_below_grade(majority_grade).powf(n);

        let p_minus_q = p - q;

        let n_a = a + 0.5 * (p_minus_q / (1.0 - p_minus_q));
        let deviation = if n_a < a { -(a - n_a) } else { a - n_a };

        info!(
            "a:{} p:{} q:{} p-q:{} n_a:{} deviation:{}",
            a, p, q, p_minus_q, n_a, deviation
        );
        deviation
    }
}

fn highest_majority_grade(tally: &[UsualJudgmentTally]) -> Grade {
    tally
        .iter()
        .map(|i| i.majority_grade())
        .max()
        .expect("No options in round")
}

fn options_majority_grade(tally: &[UsualJudgmentTally], majority_grade: Grade) -> Vec<usize> {
    tally
        .iter()
        .enumerate()
        .filter(|(_, i)| i.majority_grade() == majority_grade)
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsualJudgmentTallyCalculated {
    pub option_index: usize,
    pub ratings: [i64; 7],
    pub average_grade: Grade,
}

impl From<UsualJudgmentTally> for UsualJudgmentTallyCalculated {
    fn from(tally: UsualJudgmentTally) -> Self {
        Self {
            option_index: tally.option_index,
            ratings: tally.ratings,
            average_grade: tally.majority_grade(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Score {
    option_index: usize,
    score: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BreakTie {
    pub scores: Vec<Score>,
    pub winner: usize,
}

fn break_tie(options: &[UsualJudgmentTally]) -> BreakTie {
    // Additional tie-breaking
    for level in 1..100 {
        let next_scores: Vec<Score> = options
            .iter()
            .map(|i| {
                info!("Getting score for {:?}", i.option_index);
                Score {
                    option_index: i.option_index,
                    score: i.score_n(level as f64),
                }
            })
            .collect::<Vec<_>>();
        let best_next_score = next_scores
            .iter()
            .map(|i| i.score)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let matching_next_best_score = next_scores
            .iter()
            .filter(|i| i.score == best_next_score)
            .map(|i| i.option_index)
            .collect::<Vec<_>>();

        if matching_next_best_score.len() == 1 {
            return BreakTie {
                scores: next_scores,
                winner: matching_next_best_score[0],
            };
        }
    }

    // I really don't understand the tie breaker tie breaker
    panic!("Tie breaker tie breaker not implemented");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsualJudgmentResult {
    pub options: Vec<String>,
    pub starting_tally: Vec<UsualJudgmentTallyCalculated>,
    pub best_grade: Grade,
    pub tie_info: Option<BreakTie>,
    pub winner: usize,
    pub vote_count: usize,
    pub votes: Vec<UsualJudgmentVote>,
}

pub fn get_result(
    election: &UsualJudgmentElection,
    votes: &[UsualJudgmentVote],
) -> UsualJudgmentResult {
    let mut tally = Vec::new();
    for i in 0..election.options.len() {
        tally.push(UsualJudgmentTally {
            option_index: i,
            ratings: [0, 0, 0, 0, 0, 0, 0],
        })
    }
    for vote in votes {
        for (i, rating) in vote.votes.iter().enumerate() {
            let rating_index: usize = rating.clone().into();
            tally[i].ratings[rating_index] += 1;
        }
    }

    let best_grade = highest_majority_grade(&tally);
    let matching_best = options_majority_grade(&tally, best_grade);
    let mut tie_info = None;
    let winner = if matching_best.len() > 1 {
        let options = matching_best
            .iter()
            .map(|i| tally[*i].clone())
            .collect::<Vec<_>>();
        tie_info = Some(break_tie(&options));
        tie_info.as_ref().unwrap().winner
    } else {
        matching_best[0]
    };

    UsualJudgmentResult {
        options: election.options.clone(),
        starting_tally: tally.into_iter().map(|i| i.into()).collect(),
        best_grade,
        tie_info,
        winner,
        vote_count: votes.len(),
        votes: votes.to_vec(),
    }
}
