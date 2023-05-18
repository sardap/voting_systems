use std::collections::HashMap;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use serde::{Deserialize, Serialize};

use crate::{
    create_add_vote, create_get_votes,
    elections::{self, CreateElection, CreateElectionResult},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreElection {
    pub election: crate::models::Election,
    pub options: Vec<String>,
    pub max_score: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicScoreElection {
    pub id: String,
    pub title: String,
    pub options: Vec<String>,
    pub require_token: bool,
    pub max_score: usize,
}

impl Into<PublicScoreElection> for ScoreElection {
    fn into(self) -> PublicScoreElection {
        PublicScoreElection {
            id: self.election.id.to_string(),
            title: self.election.title.to_string(),
            options: self.options,
            require_token: self.election.requires_token,
            max_score: self.max_score,
        }
    }
}

pub fn get_election(
    c: &mut diesel::PgConnection,
    election_id: &uuid::Uuid,
) -> Option<ScoreElection> {
    let base_election = match elections::get_election(c, election_id) {
        Some(election) => election,
        None => return None,
    };

    use crate::schema::score_elections;
    let score_election: crate::models::ScoreElection = match score_elections::table
        .filter(score_elections::election_id.eq(election_id))
        .first::<crate::models::ScoreElection>(c)
        .optional()
        .unwrap()
    {
        Some(election) => election,
        None => return None,
    };

    Some(ScoreElection {
        election: base_election,
        options: score_election
            .options
            .into_iter()
            .map(|i| i.unwrap())
            .collect(),
        max_score: score_election.max_score as usize,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
    pub max_score: usize,
}

pub fn add_election(
    c: &mut diesel::PgConnection,
    arg: ScoreCreateElection,
) -> CreateElectionResult {
    let result = crate::elections::add_election(
        c,
        &arg.election_base.title,
        arg.election_base.requires_token,
    );

    use crate::schema::score_elections;
    diesel::insert_into(score_elections::table)
        .values(crate::models::ScoreElection {
            election_id: result.election_id,
            options: arg.options.into_iter().map(Some).collect(),
            max_score: arg.max_score as i32,
        })
        .execute(c)
        .unwrap();

    return result;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<usize>,
}

impl ScoreVote {
    pub fn make_model(&self, election_id: &uuid::Uuid) -> crate::models::ScoreVote {
        crate::models::ScoreVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            votes: self.votes.iter().map(|i| Some(*i as i32)).collect(),
        }
    }
}

impl From<crate::models::ScoreVote> for ScoreVote {
    fn from(v: crate::models::ScoreVote) -> Self {
        ScoreVote {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|i| i.unwrap() as usize).collect(),
        }
    }
}

create_get_votes!(
    crate::schema::score_votes,
    crate::models::ScoreVote,
    ScoreVote
);

create_add_vote!(
    crate::schema::score_votes,
    crate::models::ScoreVote,
    ScoreVote
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreTally {
    pub option_index: usize,
    pub vote_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreRunoff {
    pub participants: Vec<usize>,
    pub winners: Vec<usize>,
    pub score_checked: usize,
}

fn runoff(participants: &[usize], votes: &[ScoreVote], max_score: usize) -> ScoreRunoff {
    let mut tally: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    for i in participants {
        let mut inner_hash_map = HashMap::new();
        for j in 0..(max_score + 1) {
            inner_hash_map.insert(j, 0);
        }
        tally.insert(*i, inner_hash_map);
    }

    for vote in votes {
        for (option_index, v) in vote.votes.iter().enumerate() {
            if let Some(entry) = tally.get_mut(&option_index) {
                let col = entry.get_mut(v).unwrap();
                *col += 1;
            }
        }
    }

    let mut winners: Vec<usize>;
    let mut top = max_score;
    loop {
        let max = tally
            .iter()
            .filter(|i| i.1.get(&top).unwrap() > &0)
            .map(|i| i.1.get(&top).unwrap())
            .max()
            .unwrap();

        winners = participants
            .iter()
            .filter(|i| tally.get(i).unwrap().get(&top).unwrap() == max)
            .map(|i| *i)
            .collect();

        if winners.len() == 1 || top - 1 == 0 {
            break;
        }

        top -= 1;
    }

    ScoreRunoff {
        participants: participants.to_vec().clone(),
        winners,
        score_checked: top,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreResult {
    pub options: Vec<String>,
    pub runoff: Option<ScoreRunoff>,
    pub winner: usize,
    pub vote_tally: Vec<ScoreTally>,
    pub votes: Vec<ScoreVote>,
    pub vote_count: usize,
}

pub fn get_result(election: &ScoreElection, votes: &[ScoreVote]) -> ScoreResult {
    let mut rng: Pcg64 = Seeder::from(election.election.title.to_string()).make_rng();

    let mut vote_tally = vec![0; election.options.len()];
    for vote in votes {
        for (option_index, v) in vote.votes.iter().enumerate() {
            vote_tally[option_index] += v;
        }
    }

    let mut vote_tally: Vec<ScoreTally> = vote_tally
        .iter()
        .enumerate()
        .map(|(option_index, vote_count)| ScoreTally {
            option_index,
            vote_count: *vote_count,
        })
        .collect();

    vote_tally.sort_by(|a, b| b.vote_count.cmp(&a.vote_count));

    let top_score = vote_tally.iter().map(|i| i.vote_count).max().unwrap();
    let matching_score = vote_tally
        .iter()
        .filter(|i| i.vote_count == top_score)
        .map(|i| i.option_index)
        .collect::<Vec<_>>();

    if matching_score.len() > 1 {
        let runoff_result = runoff(&matching_score, votes, election.max_score);

        let winner = if runoff_result.winners.len() == 0 {
            runoff_result.winners[0]
        } else {
            *runoff_result.winners.choose(&mut rng).unwrap()
        };

        return ScoreResult {
            options: election.options.clone(),
            winner: winner,
            runoff: Some(runoff_result),
            vote_tally,
            votes: votes.clone().to_vec(),
            vote_count: votes.len(),
        };
    }

    ScoreResult {
        options: election.options.clone(),
        winner: matching_score[0],
        runoff: None,
        vote_tally,
        votes: votes.clone().to_vec(),
        vote_count: votes.len(),
    }
}
