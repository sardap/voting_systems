use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use serde::{Deserialize, Serialize};

use crate::{
    create_add_election, create_add_vote, create_election, create_get_election, create_get_votes,
    elections::{
        self, tally_ranked_votes, CreateElection, CreateElectionResult, RankedChoiceVote,
        RankedChoiceVoteTally,
    },
    models,
};

create_election!(ThreeTwoOneElection);

create_get_election!(crate::schema::three_two_one_elections, ThreeTwoOneElection);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreeTwoOneCreateElection {
    pub election_base: CreateElection,
    pub options: Vec<String>,
}

create_add_election!(
    ThreeTwoOneCreateElection,
    models::ThreeTwoOneElection,
    crate::schema::three_two_one_elections
);

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Hash)]
pub enum GoodOkBad {
    Good,
    Ok,
    Bad,
}

impl Eq for GoodOkBad {}

impl Into<i32> for GoodOkBad {
    fn into(self) -> i32 {
        match self {
            GoodOkBad::Bad => 0,
            GoodOkBad::Ok => 1,
            GoodOkBad::Good => 2,
        }
    }
}

impl Into<usize> for GoodOkBad {
    fn into(self) -> usize {
        match self {
            GoodOkBad::Bad => 0,
            GoodOkBad::Ok => 1,
            GoodOkBad::Good => 2,
        }
    }
}

impl From<i32> for GoodOkBad {
    fn from(val: i32) -> Self {
        match val {
            0 => GoodOkBad::Bad,
            1 => GoodOkBad::Ok,
            2 => GoodOkBad::Good,
            _ => panic!("Invalid value for GoodOkBad"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreeTwoOneVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<GoodOkBad>,
}

impl ThreeTwoOneVote {
    fn make_model(&self, election_id: &uuid::Uuid) -> models::ThreeTwoOneVote {
        models::ThreeTwoOneVote {
            id: uuid::Uuid::new_v4(),
            election_id: election_id.clone(),
            created_by: self.created_by,
            votes: self
                .votes
                .clone()
                .into_iter()
                .map(|v| Some(v.into()))
                .collect(),
        }
    }
}

impl From<crate::models::ThreeTwoOneVote> for ThreeTwoOneVote {
    fn from(v: crate::models::ThreeTwoOneVote) -> Self {
        Self {
            created_by: v.created_by,
            votes: v.votes.into_iter().map(|i| i.unwrap().into()).collect(),
        }
    }
}

impl RankedChoiceVote<usize> for ThreeTwoOneVote {
    fn ranked_votes(&self) -> Vec<usize> {
        self.votes
            .iter()
            .map(|i| {
                let x: usize = (*i).into();
                x
            })
            .collect()
    }
}

create_get_votes!(
    crate::schema::three_two_one_votes,
    models::ThreeTwoOneVote,
    ThreeTwoOneVote
);

create_add_vote!(
    crate::schema::three_two_one_votes,
    models::ThreeTwoOneVote,
    ThreeTwoOneVote
);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ThreeTwoOneTally {
    pub option_index: usize,
    pub good_count: usize,
    pub ok_count: usize,
    pub bad_count: usize,
    pub score: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreeTwoOneResult {
    pub options: Vec<String>,
    pub points_tally: Vec<ThreeTwoOneTally>,
    pub semifinalists: Vec<usize>,
    pub finalists: Vec<usize>,
    pub winner: usize,
    pub vote_count: usize,
    pub vote_tally: Vec<RankedChoiceVoteTally<usize>>,
}

fn break_semifinalist_tie(
    rng: &mut rand_pcg::Pcg64,
    tied: &[&ThreeTwoOneTally],
) -> ThreeTwoOneTally {
    let max_score = tied.iter().map(|i| i.score).max().unwrap();
    let mut matching_max_score = tied
        .iter()
        .filter(|i| i.score == max_score)
        .collect::<Vec<_>>();

    matching_max_score.shuffle(rng);
    return (*matching_max_score[0]).clone();
}

fn break_finalist_tie(rng: &mut rand_pcg::Pcg64, tied: &[&ThreeTwoOneTally]) -> ThreeTwoOneTally {
    let min_score = tied.iter().map(|i| i.score).min().unwrap();
    let mut matching_max_score = tied
        .iter()
        .filter(|i| i.score == min_score)
        .collect::<Vec<_>>();

    matching_max_score.shuffle(rng);
    return (*matching_max_score[0]).clone();
}

pub fn get_result(election: &ThreeTwoOneElection, votes: &[ThreeTwoOneVote]) -> ThreeTwoOneResult {
    // https://electowiki.org/wiki/3-2-1_voting
    let mut rng: Pcg64 = Seeder::from(election.election.title.to_string()).make_rng();

    let mut points_tally = Vec::new();
    for i in 0..election.options.len() {
        points_tally.push(ThreeTwoOneTally {
            option_index: i,
            good_count: 0,
            ok_count: 0,
            bad_count: 0,
            score: 0,
        });
    }

    for vote in votes {
        for (i, points) in vote.votes.iter().enumerate() {
            match points {
                GoodOkBad::Good => points_tally[i].good_count += 1,
                GoodOkBad::Ok => points_tally[i].ok_count += 1,
                GoodOkBad::Bad => points_tally[i].bad_count += 1,
            }

            let score: usize = points.clone().into();
            points_tally[i].score += score;
        }
    }

    // First sort by good count
    points_tally.sort_by(|a, b| a.good_count.cmp(&b.good_count).reverse());

    let mut semifinalists = Vec::new();
    {
        let mut good_tally = points_tally.clone();
        while semifinalists.len() < 3 {
            let matching_top = good_tally
                .iter()
                .filter(|i| i.good_count == good_tally[0].good_count)
                .collect::<Vec<_>>();

            let to_add = break_semifinalist_tie(&mut rng, &matching_top);
            good_tally.remove(
                good_tally
                    .iter()
                    .position(|i| i.option_index == to_add.option_index)
                    .unwrap(),
            );
            semifinalists.push(to_add);
        }
    }

    // Then sort by bad count
    semifinalists.sort_by(|a, b| b.bad_count.cmp(&a.bad_count));
    let mut finalists = Vec::new();
    {
        let mut bad_tally = semifinalists.clone();
        let matching_bottom = bad_tally
            .iter()
            .filter(|i| i.bad_count == bad_tally[0].bad_count)
            .collect::<Vec<_>>();

        let to_remove = break_finalist_tie(&mut rng, &matching_bottom);
        bad_tally.remove(
            bad_tally
                .iter()
                .position(|i| i.option_index == to_remove.option_index)
                .unwrap(),
        );
        finalists.push(bad_tally[0]);
        finalists.push(bad_tally[1]);
    }

    points_tally.sort_by(|a, b| a.score.cmp(&b.score).reverse());

    // The winner is the finalist who is rated above the other on more ballots
    let a_index = finalists[0].option_index;
    let b_index = finalists[1].option_index;
    let mut a_count = 0;
    let mut b_count = 0;
    for vote in votes {
        let a: i32 = vote.votes[a_index].into();
        let b: i32 = vote.votes[b_index].into();
        if a > b {
            a_count += 1;
        } else if b > a {
            b_count += 1;
        }
    }

    let winner = if a_count > b_count { a_index } else { b_index };

    ThreeTwoOneResult {
        vote_tally: tally_ranked_votes(votes),
        options: election.options.clone(),
        semifinalists: semifinalists.iter().map(|i| i.option_index).collect(),
        finalists: finalists.iter().map(|i| i.option_index).collect(),
        points_tally,
        winner,
        vote_count: votes.len(),
    }
}
