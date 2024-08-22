use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVElection {
    pub options: Vec<String>,
    pub elected_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVVote {
    pub created_by: uuid::Uuid,
    pub votes: Vec<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVTally {
    pub option_index: usize,
    pub vote_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SNTVResult {
    pub options: Vec<String>,
    pub winners: Vec<usize>,
    pub vote_tally: Vec<SNTVTally>,
    pub votes: Vec<SNTVVote>,
    pub vote_count: usize,
}

pub fn get_result(election: &SNTVElection, votes: &[SNTVVote]) -> SNTVResult {
    let mut vote_tally = vec![0; election.options.len()];
    for vote in votes {
        for (option_index, v) in vote.votes.iter().enumerate() {
            if *v {
                vote_tally[option_index] += 1;
            }
        }
    }

    let mut vote_tally: Vec<SNTVTally> = vote_tally
        .iter()
        .enumerate()
        .map(|(option_index, vote_count)| SNTVTally {
            option_index,
            vote_count: *vote_count,
        })
        .collect();

    vote_tally.sort_by(|a, b| b.vote_count.cmp(&a.vote_count));

    let winners: Vec<usize> = vote_tally.clone()[0..election.elected_count]
        .to_vec()
        .iter()
        .map(|i| i.option_index)
        .collect();

    SNTVResult {
        options: election.options.clone(),
        winners,
        vote_tally: vote_tally.clone(),
        votes: votes.to_vec(),
        vote_count: votes.len(),
    }
}
