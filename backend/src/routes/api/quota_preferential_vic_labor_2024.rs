use actix_web::{
    post,
    web::{self},
    HttpResponse, Scope,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use voting_systems::quota_preferential_vic_labor_2024::{get_result, ElectionResult, Vote};

use voting_systems::quota_preferential_vic_labor_2024::{Candidate, Election};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CandidateRequest {
    #[validate(length(min = 1, max = 100))]
    name: String,
    is_female: bool,
}

fn validate_election_request(election: &ElectionRequest) -> Result<(), ValidationError> {
    if election.elected_count > election.candidates.len() {
        return Err(ValidationError::new(
            "elected_count must be less than or equal to the number of candidates",
        ));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_election_request", skip_on_field_errors = false))]
pub struct ElectionRequest {
    #[validate(length(min = 2, max = 100))]
    candidates: Vec<CandidateRequest>,
    #[validate(range(min = 0.0, max = 1.0))]
    percent_female: f64,
    #[validate(range(min = 1, max = 100))]
    elected_count: usize,
}

impl Into<Election> for ElectionRequest {
    fn into(self) -> Election {
        Election {
            candidates: self
                .candidates
                .into_iter()
                .map(|c| Candidate::new(c.name, c.is_female))
                .collect(),
            percent_female: self.percent_female,
            elected_count: self.elected_count,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VoteBundle {
    #[validate(range(min = 1, max = 100))]
    count: usize,
    vote: Vec<usize>,
}

fn validate_test_election_request(election: &TestElectionRequest) -> Result<(), ValidationError> {
    for bundle in &election.bundles {
        for pref in &bundle.vote {
            if *pref > election.election.candidates.len() {
                return Err(ValidationError::new("Invalid preference in vote"));
            }
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(
    function = "validate_test_election_request",
    skip_on_field_errors = false
))]
pub struct TestElectionRequest {
    #[validate]
    election: ElectionRequest,
    #[validate(length(min = 1, max = 200))]
    bundles: Vec<VoteBundle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectionResultResponse {
    vote_count: usize,
    candidates: Vec<String>,
    votes: Vec<VoteBundle>,
    result: ElectionResult,
}

#[post("/test")]
async fn post_test_election(
    request: actix_web_validator::Json<TestElectionRequest>,
) -> actix_web::HttpResponse {
    let request = request.into_inner();

    let mut votes = vec![];
    for bundle in &request.bundles {
        for _ in 0..bundle.count {
            let vote = Vote::new_unknown(bundle.vote.clone());
            votes.push(vote);
        }
    }

    let election: Election = request.election.into();

    let result = get_result(&election, &votes);

    HttpResponse::Ok().json(ElectionResultResponse {
        vote_count: votes.len(),
        candidates: election.candidates.iter().map(|c| c.name.clone()).collect(),
        votes: request.bundles,
        result,
    })
}

pub fn routes() -> Scope {
    web::scope("/quota_preferential_vic_labor_2024").service(post_test_election)
}
