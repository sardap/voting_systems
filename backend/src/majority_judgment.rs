use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult},
};

create_election!(MajorityJudgmentElection);

create_get_election!(
    crate::schema::majority_judgment_elections,
    MajorityJudgmentElection
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MajorityJudgmentCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
}

create_add_election!(
    MajorityJudgmentCreateElection,
    crate::models::MajorityJudgmentElection,
    crate::schema::majority_judgment_elections
);

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Ord, PartialOrd)]
pub enum Rating {
    Terrible,
    Poor,
    Acceptable,
    Good,
    VeryGood,
}

impl Eq for Rating {}

impl Into<i32> for Rating {
    fn into(self) -> i32 {
        match self {
            Rating::VeryGood => 4,
            Rating::Good => 3,
            Rating::Acceptable => 2,
            Rating::Poor => 1,
            Rating::Terrible => 0,
        }
    }
}

impl Into<usize> for Rating {
    fn into(self) -> usize {
        match self {
            Rating::VeryGood => 4,
            Rating::Good => 3,
            Rating::Acceptable => 2,
            Rating::Poor => 1,
            Rating::Terrible => 0,
        }
    }
}

impl From<i32> for Rating {
    fn from(val: i32) -> Self {
        match val {
            4 => Rating::VeryGood,
            3 => Rating::Good,
            2 => Rating::Acceptable,
            1 => Rating::Poor,
            0 => Rating::Terrible,
            _ => panic!("Invalid rating"),
        }
    }
}

impl From<usize> for Rating {
    fn from(val: usize) -> Self {
        match val {
            4 => Rating::VeryGood,
            3 => Rating::Good,
            2 => Rating::Acceptable,
            1 => Rating::Poor,
            0 => Rating::Terrible,
            _ => panic!("Invalid rating"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MajorityJudgmentVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<Rating>,
}

impl MajorityJudgmentVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> crate::models::MajorityJudgmentVote {
        crate::models::MajorityJudgmentVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            votes: self
                .votes
                .clone()
                .into_iter()
                .map(|r| Some(r.into()))
                .collect(),
        }
    }
}

impl From<crate::models::MajorityJudgmentVote> for MajorityJudgmentVote {
    fn from(vote: crate::models::MajorityJudgmentVote) -> Self {
        MajorityJudgmentVote {
            created_by: vote.created_by,
            votes: vote
                .votes
                .into_iter()
                .map(|r| r.map(|r| r.into()).unwrap_or(Rating::Terrible))
                .collect(),
        }
    }
}

create_get_votes!(
    crate::schema::majority_judgment_votes,
    crate::models::MajorityJudgmentVote,
    MajorityJudgmentVote
);

create_add_vote!(
    crate::schema::majority_judgment_votes,
    crate::models::MajorityJudgmentVote,
    MajorityJudgmentVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MajorityJudgmentTally {
    pub option_index: usize,
    pub ratings: [i64; 5],
}

impl MajorityJudgmentTally {
    pub fn median(&self) -> Rating {
        //TODO change this to use math not memory
        let mut big_vec = self
            .ratings
            .iter()
            .enumerate()
            .map(|(k, v)| {
                let rating: Rating = k.into();
                vec![rating; *v as usize]
            })
            .flatten()
            .collect::<Vec<_>>();
        big_vec.sort_by(|a, b| a.cmp(b));

        big_vec[big_vec.len() / 2]
    }
}

fn highest_median(tally: &[MajorityJudgmentTally]) -> Rating {
    tally
        .iter()
        .map(|i| i.median())
        .max()
        .expect("No options in round")
}

fn options_matching_median(tally: &[MajorityJudgmentTally], median: Rating) -> Vec<usize> {
    tally
        .iter()
        .enumerate()
        .filter(|(_, i)| i.median() == median)
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MajorityJudgmentRunoff {
    pub modified_tally: Vec<MajorityJudgmentTally>,
    pub best_median: Rating,
    pub participants: Vec<usize>,
    pub winners: Vec<usize>,
}

fn find_winners(starting_tally: &[MajorityJudgmentTally]) -> MajorityJudgmentRunoff {
    let best_median = highest_median(&starting_tally);
    let best_median_idx: usize = best_median.into();
    let mut modified_tally = starting_tally.to_vec();
    let mut old_tally = modified_tally.clone();
    let mut options_matching;
    loop {
        options_matching = options_matching_median(&modified_tally, best_median);

        if options_matching.len() == 0 {
            return MajorityJudgmentRunoff {
                modified_tally,
                best_median,
                participants: options_matching_median(starting_tally, best_median),
                winners: options_matching_median(&old_tally, best_median),
            };
        }

        if options_matching.len() == 1 {
            return MajorityJudgmentRunoff {
                modified_tally,
                best_median,
                participants: options_matching_median(starting_tally, best_median),
                winners: vec![options_matching[0]],
            };
        }

        // Subtract one from the selected median for all options
        old_tally = modified_tally.clone();
        for option in &options_matching {
            let tally = modified_tally.get_mut(*option).unwrap();
            let rating = tally.ratings.get_mut(best_median_idx).unwrap();
            if *rating - 1 < 1 {
                continue;
            }
            *rating -= 1;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MajorityJudgmentResult {
    pub options: Vec<String>,
    pub starting_tally: Vec<MajorityJudgmentTally>,
    pub best_median: Rating,
    pub runoff: MajorityJudgmentRunoff,
    pub score_result: Option<crate::score::ScoreResult>,
    pub winner: usize,
    pub vote_count: usize,
    pub votes: Vec<MajorityJudgmentVote>,
}

pub fn get_result(
    election: &MajorityJudgmentElection,
    votes: &[MajorityJudgmentVote],
) -> MajorityJudgmentResult {
    return MajorityJudgmentResult {
        options: election.options.clone(),
        winner: 0,
        vote_count: 0,
        votes: Vec::new(),
        best_median: Rating::Terrible,
        starting_tally: Vec::new(),
        runoff: MajorityJudgmentRunoff {
            modified_tally: Vec::new(),
            best_median: Rating::Terrible,
            participants: Vec::new(),
            winners: Vec::new(),
        },
        score_result: None,
    };

    let mut tally = Vec::new();
    for i in 0..election.options.len() {
        tally.push(MajorityJudgmentTally {
            option_index: i,
            ratings: [0, 0, 0, 0, 0],
        })
    }
    for vote in votes {
        for (i, rating) in vote.votes.iter().enumerate() {
            let rating_index: usize = rating.clone().into();
            tally[i].ratings[rating_index] += 1;
        }
    }

    let runoff = find_winners(&tally);
    if runoff.winners.len() > 1 {
        let score_election = crate::score::ScoreElection {
            election: election.election.clone(),
            options: election.options.clone(),
            max_score: Rating::VeryGood.into(),
        };
        let score_votes = votes
            .iter()
            .map(|i| crate::score::ScoreVote {
                created_by: i.created_by,
                votes: i
                    .votes
                    .iter()
                    .enumerate()
                    .map(|(option_index, j)| {
                        if runoff.winners.contains(&option_index) {
                            j.clone().into()
                        } else {
                            0
                        }
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();
        let score_result = crate::score::get_result(&score_election, &score_votes);

        return MajorityJudgmentResult {
            options: election.options.clone(),
            winner: score_result.winner,
            vote_count: votes.len(),
            votes: votes.to_vec(),
            best_median: highest_median(&tally),
            starting_tally: tally,
            runoff,
            score_result: Some(score_result),
        };
    };

    let winner = runoff.winners[0];

    MajorityJudgmentResult {
        options: election.options.clone(),
        winner,
        vote_count: votes.len(),
        votes: votes.to_vec(),
        best_median: highest_median(&tally),
        starting_tally: tally,
        runoff,
        score_result: None,
    }
}
