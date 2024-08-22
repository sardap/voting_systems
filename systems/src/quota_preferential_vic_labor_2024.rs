use std::{collections::HashMap, fmt::format};

use serde_derive::{Deserialize, Serialize};

const PAPER_SCORE: i64 = 1000;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    created_by: uuid::Uuid,
    votes: Vec<usize>,
}

impl Vote {
    pub fn new(created_by: uuid::Uuid, votes: Vec<usize>) -> Vote {
        Vote { created_by, votes }
    }

    pub fn new_unknown(votes: Vec<usize>) -> Vote {
        Vote {
            created_by: uuid::Uuid::new_v4(),
            votes,
        }
    }

    pub fn top_continuing_preference(&self, continuing_candidates: &[usize]) -> Option<usize> {
        self.votes
            .iter()
            .find(|i| continuing_candidates.contains(i))
            .copied()
    }

    pub fn candidate_preference(&self, candidate: usize) -> Option<usize> {
        self.votes.iter().position(|i| *i == candidate)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candidate {
    pub name: String,
    pub is_female: bool,
}

impl Candidate {
    pub fn new<T: ToString>(name: T, is_female: bool) -> Candidate {
        Candidate {
            name: name.to_string(),
            is_female,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Election {
    pub candidates: Vec<Candidate>,
    pub percent_female: f64,
    pub elected_count: usize,
}

impl Election {
    pub fn quota<T: Into<usize>>(&self, total_votes: T) -> i64 {
        ((total_votes.into() * PAPER_SCORE as usize) / (self.candidates.len() + 1)) as i64
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paper {
    pub vote: Vote,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountSheetEntry {
    pub option_index: usize,
    pub papers: Vec<Paper>,
    pub separated_papers: Vec<uuid::Uuid>,
    pub transfer_value: i64,
    pub remainders: i64,
}

impl CountSheetEntry {
    pub fn new(option_index: usize) -> CountSheetEntry {
        CountSheetEntry {
            option_index,
            papers: vec![],
            separated_papers: vec![],
            transfer_value: 0,
            remainders: 0,
        }
    }

    pub fn score(&self) -> i64 {
        self.papers.iter().map(|i| i.value).sum::<i64>()
    }

    pub fn add_vote(&mut self, vote: Vote, value: i64) {
        self.papers.push(Paper { vote, value });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountSheet {
    pub entries: HashMap<usize, CountSheetEntry>,
    pub exhausted_papers: Vec<Paper>,
}

impl CountSheet {
    pub fn new(candidates: &[usize]) -> CountSheet {
        CountSheet {
            entries: HashMap::from_iter(candidates.iter().map(|i| (*i, CountSheetEntry::new(*i)))),
            exhausted_papers: vec![],
        }
    }

    pub fn get(&self, option_index: usize) -> &CountSheetEntry {
        self.entries.get(&option_index).unwrap()
    }

    pub fn get_mut(&mut self, option_index: usize) -> &mut CountSheetEntry {
        self.entries.get_mut(&option_index).unwrap()
    }

    pub fn top_score(&self, candidates: &[usize]) -> i64 {
        self.entries
            .iter()
            .filter(|i| candidates.contains(i.0))
            .map(|i| i.1.score())
            .max()
            .unwrap()
    }

    pub fn lowest_score(&self, candidates: &[usize]) -> i64 {
        self.entries
            .iter()
            .filter(|i| candidates.contains(i.0))
            .map(|i| i.1.score())
            .min()
            .unwrap()
    }

    pub fn matching_score(&self, candidates: &[usize], score: i64) -> Vec<usize> {
        self.entries
            .iter()
            .filter(|i| i.1.score() == score && candidates.contains(i.0))
            .map(|i| *i.0)
            .collect()
    }

    pub fn add_vote(&mut self, option_index: usize, vote: Vote, value: i64) {
        self.entries
            .get_mut(&option_index)
            .unwrap()
            .add_vote(vote, value);
    }

    pub fn transfer_paper(&mut self, from: usize, to: usize, paper: &Paper, value: i64) {
        let real_paper = self.remove_paper(from, paper);
        self.add_vote(to, real_paper.vote, value);
    }

    pub fn transfer_paper_4_2(&mut self, from: usize, to: usize, paper: &Paper, value: i64) {
        let real_paper = self.remove_paper(from, paper);
        self.entries
            .get_mut(&from)
            .unwrap()
            .separated_papers
            .push(real_paper.vote.created_by);
        self.add_vote(to, real_paper.vote, value);
    }

    pub fn remove_paper(&mut self, option_index: usize, paper: &Paper) -> Paper {
        let entry = self.entries.get_mut(&option_index).unwrap();
        entry.papers.remove(
            entry
                .papers
                .iter()
                .position(|i| i.vote.created_by == paper.vote.created_by)
                .unwrap(),
        )
    }

    pub fn exhaust_paper(&mut self, option_index: usize, paper: &Paper) {
        let real_paper = self.remove_paper(option_index, paper);
        self.exhausted_papers.push(real_paper);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountSheetCollection {
    pub count_sheets: Vec<CountSheet>,
    pub aa_count_sheets: Vec<CountSheet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecisionLog {
    #[serde(skip)]
    pub candidate_names: Vec<String>,
    pub decisions: Vec<String>,
}

impl DecisionLog {
    pub fn new(election: &Election) -> DecisionLog {
        DecisionLog {
            candidate_names: election.candidates.iter().map(|i| i.name.clone()).collect(),
            decisions: vec![],
        }
    }

    pub fn add<T: ToString>(&mut self, decision: T) {
        // Regex for Arrays Should be lazy static
        let array_re = regex::Regex::new(r"\$C\[(.*)\]").unwrap();
        let decision = decision.to_string();
        let decision = array_re.replace_all(&decision, |caps: &regex::Captures| {
            let mut candidates: Vec<usize> = vec![];

            if let Some(m) = caps.get(1) {
                let splits = m.as_str().split(", ");
                for split in splits {
                    let split = match split.parse::<usize>() {
                        Ok(i) => i,
                        Err(_) => {
                            return format!("[{}]", split);
                        }
                    };
                    candidates.push(split);
                }
            }

            let names = candidates
                .iter()
                .map(|i| self.candidate_names[*i].clone())
                .collect::<Vec<_>>();

            format!("[{}]", names.join(", "))
        });
        let candidate_re = regex::Regex::new(r"\$C\d+").unwrap();
        let decision = candidate_re.replace_all(&decision, |caps: &regex::Captures| {
            let candidate = caps.get(0).unwrap().as_str();
            let candidate = candidate[2..].parse::<usize>().unwrap();
            self.candidate_names[candidate].clone()
        });

        self.decisions.push(decision.to_string());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoteBundle {
    pub vote: Vote,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElectionResult {
    pub elected_candidates: Vec<usize>,
    pub quota: i64,
    pub count_sheet: CountSheetCollection,
    pub decision_log: DecisionLog,
}

fn distribute_surplus(
    decision_log: &mut DecisionLog,
    active_cs: &mut CountSheet,
    quota: i64,
    continuing_candidates: &[usize],
    to_distribute: usize,
) {
    let mut papers = active_cs.get_mut(to_distribute).papers.clone();

    papers.sort_by(|a, b| {
        let a_next_preference = a.vote.top_continuing_preference(&continuing_candidates);
        let b_next_preference = b.vote.top_continuing_preference(&continuing_candidates);
        a_next_preference.cmp(&b_next_preference)
    });

    decision_log.add(format!(
        "As per 3.1 re-sorted the papers for $C{} by next preference",
        to_distribute
    ));

    let score = active_cs.get_mut(to_distribute).score();
    let surplus = score - quota as i64;
    if surplus <= 0 {
        decision_log.add(format!(
            "As per 3.7 no surplus for $C{} meaning nothing to do",
            to_distribute
        ));
        return;
    }

    let total_ballots = papers.len() as i64;
    // Divide the surplus by the number of papers, including the exhaustive papers
    // Whole number part is the transfer value
    let transfer_value = surplus / total_ballots;

    let mut transfer_tally = HashMap::new();
    for paper in papers {
        if let Some(next_top) = paper.vote.top_continuing_preference(&continuing_candidates) {
            *transfer_tally.entry(next_top as i32).or_insert(0) += 1;
            active_cs.transfer_paper(to_distribute, next_top, &paper, transfer_value);
        } else {
            *transfer_tally.entry(-1).or_insert(0) += 1;
            active_cs.exhaust_paper(to_distribute, &paper);
        }
    }

    for (candidate, count) in transfer_tally {
        if candidate == -1 {
            decision_log.add(format!("Exhausted {} votes", count));
        } else {
            decision_log.add(format!(
                "Transferred {}({}) votes to $C{}",
                count,
                count * transfer_value,
                candidate
            ));
        }
    }
}

fn fill_aa_requirements(
    decision_log: &mut DecisionLog,
    election: &Election,
    votes: &[Vote],
    elected_candidates: &[usize],
    defeated_candidates: &[usize],
    active_cs: &mut CountSheet,
    backup_candidates: usize,
) -> Vec<usize> {
    decision_log.add("Filling AA requirements");

    /*
       7. To meet the affirmative action requirements of Rule 3, in all elections the Returning Officer must
       ensure that the minimum percentage of positions are filled by women. If the calculation to
       determine the minimum percentage results in a fraction of more than one half then the minimum
       percentage shall be the next higher whole number, and where it results in a fraction of one half or
       less it shall be the next lower number. This shall be achieved by the following procedure:
    */

    /*
        7.1. Immediately before recording as elected any male candidate, the Returning Officer must
        calculate whether the election of that candidate would be contrary to the Affirmative Action
        requirements. If so, the Returning Officer must reverse the last distribution of papers.
    */

    /*
       7.2. Exclude the remaining male candidates
    */

    let mut continuing_candidates = (0..election.candidates.len())
        .filter(|i| election.candidates[*i].is_female && !elected_candidates.contains(i))
        .collect::<Vec<_>>();
    let mut newly_elected = vec![];
    decision_log.add(format!(
        "continuing AA candidates $C{:?}",
        continuing_candidates
    ));

    /*
        7.3. Take all the papers off the table (including those of unelected female candidates).
    */
    // Already done since this is a program and I have no real papers

    /*
        7.4. Make a new place marker for each female candidate not yet elected, and a new count-sheet
        for the affirmative action count carrying forward previous quotas, remainders, etc. (note: all
        such candidates are reintroduced to the count at zero votes, regardless of how many votes
        they had earlier in the count), then re-introduce any defeated female candidates into the
        count at a zero number of points.
    */
    let quota = election.quota(votes.len());

    /*
        7.5. Distribute any surpluses not previously distributed from the already elected candidates, in
        the order they were declared elected, between the female candidates on the table. If a
        candidate is elected on this surplus, she is not allocated any further ballot papers.
    */
    decision_log.add("As Per 7.5 distributing surplus of the elected candidates");
    for elected_candidate in elected_candidates {
        let mut transfer_log = HashMap::new();
        let elected_candidate = *elected_candidate;

        let entry = active_cs.get(elected_candidate);

        let surplus = entry.score() - quota as i64;

        if surplus <= 0 {
            continue;
        }

        let total_ballots = entry.papers.len() as i64;
        // Divide the surplus by the number of papers, including the exhaustive papers
        // Whole number part is the transfer value
        let transfer_value = surplus / total_ballots;

        let papers = entry.papers.clone();
        decision_log.add(format!(
            "As per 7.5 distributing surplus of $C{} with a surplus of {}",
            elected_candidate, surplus
        ));

        for paper in papers {
            if let Some(next_top) = paper.vote.top_continuing_preference(&continuing_candidates) {
                active_cs.transfer_paper(elected_candidate, next_top, &paper, transfer_value);
                *transfer_log.entry(next_top).or_insert(0) += transfer_value;

                // as per 7.5 stop if they reach the quota
                if active_cs.get(next_top).score() >= quota {
                    decision_log.add(format!("Candidate $C{} has reached the quota", next_top));
                    newly_elected.push(next_top);
                    continuing_candidates.retain(|i| *i != next_top);
                }
            }
        }

        for (candidate, count) in transfer_log {
            decision_log.add(format!(
                "Transferred {}({}) votes to $C{}",
                count, count, candidate
            ));
        }
    }

    // Current counts
    for candidate in &continuing_candidates {
        decision_log.add(format!(
            "Candidate $C{} has a score of {}",
            candidate,
            active_cs.get(*candidate).score()
        ));
    }

    /*
        7.6. Re-distribute any previously exhausted papers bundle by bundle in the order in which they
        were previously declared exhausted. Each continuing female candidate is allotted those
        papers on which she is indicated as preferred to any other continuing female candidate, at
        the same values at which the papers were originally excluded, keeping every bundle of papers
        separate. Once a candidate reaches the quota she is not allocated any further ballot papers
        beyond the bundle that elected her.
    */
    decision_log.add("As per 7.6 re-distributing exhausted papers");
    for paper in active_cs.exhausted_papers.clone() {
        if let Some(next_top) = paper.vote.top_continuing_preference(&continuing_candidates) {
            active_cs.add_vote(next_top, paper.vote.clone(), paper.value);

            // as per 7.6 stop if they reach the quota
            if active_cs.get(next_top).score() >= quota {
                decision_log.add(format!("Candidate $C{} has reached the quota", next_top));
                newly_elected.push(next_top);
                continuing_candidates.retain(|i| *i != next_top);
            }
        }
    }

    // Current counts
    for candidate in &continuing_candidates {
        decision_log.add(format!(
            "Candidate $C{} has a score of {}",
            candidate,
            active_cs.get(*candidate).score()
        ));
    }

    /*
        7.7. Re-distribute in an order determined by lot the papers of any female candidate still in the
        count at the time the Rule in clause 7.1 of this Schedule was applied, bundle by bundle in the
        order in which they were received by the female candidate. Each continuing female candidate
        is allotted those papers on which she is indicated as preferred to any other continuing female
        candidate, at the same values at which the papers were originally received by the first
        mentioned female candidate, keeping every bundle of papers separate. Once a candidate
        reaches the quota she is not allocated any further ballot papers beyond the bundle that
        elected her
    */
    decision_log.add("As per 7.7 re-distributing papers of non elected female candidates to higher rated female candidates");
    let female_cc_as_start = continuing_candidates
        .iter()
        .filter(|i| !defeated_candidates.contains(i))
        .cloned()
        .collect::<Vec<_>>();
    for candidate in female_cc_as_start {
        let mut transfer_log = HashMap::new();
        // as per 7.7 transfer papers from the existing candidates if an eliminated female candidates is higher on the list
        let papers = active_cs.get(candidate).papers.clone();
        for paper in papers {
            if let Some(next_top) = paper.vote.top_continuing_preference(&continuing_candidates) {
                active_cs.transfer_paper(candidate, next_top, &paper, paper.value);
                *transfer_log.entry(next_top).or_insert(0) += paper.value;

                // as per 7.7 stop if they reach the quota
                if active_cs.get(next_top).score() >= quota {
                    decision_log.add(format!("Candidate $C{} has reached the quota", next_top));
                    newly_elected.push(next_top);
                    continuing_candidates.retain(|i| *i != next_top);
                }
            }
        }

        for (candidate, count) in transfer_log {
            decision_log.add(format!(
                "$C{} Transferred {} votes to $C{}",
                candidate, count, candidate
            ));
        }
    }

    // Current counts
    for candidate in &continuing_candidates {
        decision_log.add(format!(
            "Candidate $C{} has a score of {}",
            candidate,
            active_cs.get(*candidate).score()
        ));
    }

    /*
        7.8. Re-distribute in an order determined by lot the papers of the excluded male candidate,
        bundle by bundle in the order in which they were received by the male candidate. Each
        continuing female candidate is allotted those papers on which she is indicated as preferred
        to any other continuing female candidate, at the same values at which the papers were
        originally received by the excluded male candidate, keeping every bundle of papers separate.
        Once a candidate reaches the quota she is not allocated any further ballot papers beyond the
        bundle that elected her.
    */
    decision_log.add("As per 7.8 re-distributing papers of non elected male candidates");
    let excluded_male_cc = (0..election.candidates.len())
        .filter(|i| !elected_candidates.contains(&i) && !election.candidates[*i].is_female)
        .collect::<Vec<_>>();
    for candidate in excluded_male_cc {
        let mut transfer_log = HashMap::new();

        let papers = active_cs.get(candidate).papers.clone();
        for paper in papers {
            if let Some(next_top) = paper.vote.top_continuing_preference(&continuing_candidates) {
                active_cs.transfer_paper(candidate, next_top, &paper, paper.value);
                *transfer_log.entry(next_top).or_insert(0) += paper.value;

                // as per 7.8 stop if they reach the quota
                if active_cs.get(next_top).score() >= quota {
                    decision_log.add(format!("Candidate $C{} has reached the quota", next_top));
                    newly_elected.push(next_top);
                    continuing_candidates.retain(|i| *i != next_top);
                }
            }
        }

        for (candidate, count) in transfer_log {
            decision_log.add(format!(
                "$C{} Transferred {} value to $C{}",
                candidate, count, candidate
            ));
        }
    }

    /*
        7.9. After all the votes taken off the table under this clause have been allocated, commence the
        distribution of any surpluses of the women elected by virtue of the Affirmative Action
        provisions.
    */
    if newly_elected.len() > 0 {
        decision_log.add(format!(
            "The following females have reached the quota $C{:?}",
            newly_elected
        ));

        decision_log.add(format!(
            "As per 7.9 Distributing surplus of the elected candidates $C{:?}",
            newly_elected
        ));

        let continuing_candidates: Vec<_> = get_continuing_candidates(
            election.candidates.len(),
            &elected_candidates,
            &defeated_candidates,
        )
        .into_iter()
        .filter(|i| !newly_elected.contains(i))
        .collect();

        if !continuing_candidates.is_empty() {
            for candidate in &newly_elected {
                distribute_surplus(
                    decision_log,
                    active_cs,
                    quota,
                    &continuing_candidates,
                    *candidate,
                );
            }
        }

        if newly_elected.len() + elected_candidates.len() > election.elected_count {
            let number_needed = election.elected_count - elected_candidates.len();
            decision_log
                .add("Too many candidates elected in AA removing them in order they were selected");
            loop {
                let removed = newly_elected.pop();
                if let Some(removed) = removed {
                    decision_log.add(format!("Removing $C{}", removed));
                }

                if newly_elected.len() == number_needed {
                    break;
                }
            }

            decision_log.add(format!(
                "Removed excess candidates the following will be elected $C{:?}",
                newly_elected
            ));
        }
    } else {
        decision_log.add(
format!(
                "After an attempt of following the AA requirements in 7 no candidates were elected in there place electing the original non aa candidate $C{}",
                backup_candidates
            )
        );
        newly_elected.push(backup_candidates);
    }

    return newly_elected;
}

fn get_continuing_candidates(
    candidate_count: usize,
    elected_candidates: &[usize],
    defeated_candidates: &[usize],
) -> Vec<usize> {
    (0..candidate_count)
        .filter(|i| !elected_candidates.contains(i) && !defeated_candidates.contains(i))
        .collect::<Vec<_>>()
}

pub fn get_result(election: &Election, votes: &[Vote]) -> ElectionResult {
    let mut decision_log = DecisionLog::new(election);

    if election.candidates.len() <= election.elected_count {
        decision_log.add(
            "Candidates are less than or equal to the number of vacancies. Everyone wins! Yay!",
        );
        return ElectionResult {
            elected_candidates: (0..election.candidates.len()).collect::<Vec<_>>(),
            quota: 0,
            count_sheet: CountSheetCollection {
                count_sheets: vec![],
                aa_count_sheets: vec![],
            },
            decision_log,
        };
    }

    /*
       7. To meet the affirmative action requirements of Rule 3, in all elections the Returning Officer must
       ensure that the minimum percentage of positions are filled by women. If the calculation to
       determine the minimum percentage results in a fraction of more than one half then the minimum
       percentage shall be the next higher whole number, and where it results in a fraction of one half or
       less it shall be the next lower number.
    */
    decision_log.add(format!(
        "as per 7 finding the minimum number of female positions percent set to {}",
        election.percent_female
    ));

    let min_female_positions = {
        let female_positions = (election.elected_count as f64) * election.percent_female;
        decision_log.add(format!(
            "as per 7 calculated minimum number of female positions {}",
            female_positions
        ));

        let min_count = if female_positions.fract() > 0.5 {
            decision_log.add("as per 7 Since the fraction is greater than 0.5 rounding up");
            female_positions.ceil() as usize
        } else {
            decision_log
                .add("as per 7 Since the fraction is less than or equal to 0.5 rounding down");
            female_positions.floor() as usize
        };

        decision_log.add(format!(
            "as per 7 minimum female determined positions {}",
            min_count
        ));
        // Check enough females exist
        let female_candidate_count = election.candidates.iter().filter(|i| i.is_female).count();
        if min_count > female_candidate_count {
            decision_log.add(format!("However since there are only {} women candidates the minimum will be set to that number", female_candidate_count));
            female_candidate_count
        } else {
            min_count
        }
    };

    let mut elected_candidates: Vec<usize> = vec![];
    let mut defeated_candidates: Vec<usize> = vec![];
    let mut aa_round_sheets = vec![];

    /*
        2.2 Sort the formal papers according to the first preferences shown for each candidate and record
        the tallies of each in the respective columns of the “Count Sheet”, with each voting paper
        having the value of 1,000 points. Electronic Count Sheets approved by the Administrative
        Committee may be used in place of a physical count sheet. Any candidate will be permitted
        to scrutinise a physical or electronic count sheet. A copy of the count sheet will be provided
        to a candidate upon request.

        AKA sort papers by first preference and record the tallies
    */
    let mut votes = votes.to_vec();

    let mut active_cs = CountSheet::new(&(0..election.candidates.len()).collect::<Vec<_>>());

    // Sort the votes by first preference
    votes.sort_by(|a, b| {
        let a_first_preference = a.votes[0];
        let b_first_preference = b.votes[0];
        a_first_preference.cmp(&b_first_preference)
    });

    // Record the tallies
    for vote in &votes {
        let first_preference = vote.votes[0];
        active_cs.add_vote(first_preference, vote.clone(), PAPER_SCORE);
    }

    /*
       2.3 Total the number of formal votes and work out the “quota” in accordance with the following
       formula: (Total number of formal votes cast x 1,000 points) / (Number of vacancies to be filled + 1)
       The result obtained is taken to the next whole figure, which becomes the “quota”.
       Example: To elect six candidates from a total of 150 formal votes polled.
       (150 x 1000) / (6 + 1) = 21428 4/7 = 21,429 points
       Note: this formula provides that only the required number of vacancies can receive a “quota” of points.
    */

    let total_votes = votes.len() as i64;
    let quota = election.quota(total_votes as usize);

    decision_log.add(format!("Total votes: {} Quota: {}", total_votes, quota));

    let elected_female_count = |elected_candidates: &[usize]| {
        elected_candidates
            .iter()
            .filter(|i| election.candidates[**i].is_female)
            .count()
    };
    let aa_possible = |elected_candidates: &[usize]| {
        let female_count = elected_female_count(elected_candidates) as i64;
        let females_needed = min_female_positions as i64 - female_count;
        let candidates_needed =
            election.elected_count as i64 - (elected_candidates.len() as i64 + 1);
        return candidates_needed >= females_needed;
    };
    let get_continuing_candidates =
        |elected_candidates: &[usize], defeated_candidates: &[usize]| {
            (0..election.candidates.len())
                .filter(|i| !elected_candidates.contains(i) && !defeated_candidates.contains(i))
                .collect::<Vec<_>>()
        };

    /*
        2.4. Subject to the Affirmative Action principles of clause 7 of this Schedule, record as elected all
        candidates whose value of first preference papers equals or exceeds the quota, in order of
        their election, commencing with the highest number of points polled, then next and so on. In
        the circumstances where is an equality of the value of first preference papers, the Returning
        Officer shall determine the order by lot.
    */

    /*
        7. To meet the affirmative action requirements of Rule 3, in all elections the Returning Officer must
        ensure that the minimum percentage of positions are filled by women. If the calculation to
        determine the minimum percentage results in a fraction of more than one half then the minimum
        percentage shall be the next higher whole number, and where it results in a fraction of one half or
        less it shall be the next lower number. This shall be achieved by the following procedure:
    */

    decision_log.add("As per 2.4 checking for candidates over quota");
    while elected_candidates.len() < election.elected_count {
        let continuing_candidates =
            get_continuing_candidates(&elected_candidates, &defeated_candidates);

        let top_score = active_cs.top_score(&continuing_candidates);

        // Is the top candidate over quota? and a woman?
        // The top candidate has reached quota
        // As per 2.4 this needs to be equal or exceed the quota
        if top_score >= quota {
            let mut candidates_with_top_score: Vec<_> =
                active_cs.matching_score(&continuing_candidates, top_score);

            decision_log.add(format!(
                "Top score: {} held by $C{:?} is over the quota",
                top_score, candidates_with_top_score
            ));

            if candidates_with_top_score.len() > 1 {
                decision_log
                    .add("Multiple candidates with the same score selecting alphabetically");
                // As per 2.4 where is an equality of the value of first preference papers, the Returning Officer shall determine the order by lot.
                // To simulate this we just sort the candidates alphabetically
                // Change to always pick a female for simplicity
                candidates_with_top_score.sort_by(|a, b| {
                    election.candidates[*a]
                        .name
                        .cmp(&election.candidates[*b].name)
                });
            }

            let top_candidate = candidates_with_top_score[0];
            decision_log.add(format!("Top candidate selected: $C{}", top_candidate));

            // Ref 7.1
            if !election.candidates[top_candidate].is_female && !aa_possible(&elected_candidates) {
                decision_log.add(format!(
                    "As per 7 Since electing candidate $C{} would breaking",
                    top_candidate
                ));
                break;
            } else {
                decision_log.add("Electing candidate would not violate AA requirements");
                // Elect the candidate
                elected_candidates.push(top_candidate);
            }
        } else {
            decision_log.add("No candidates over quota");
            break;
        }
    }

    if elected_candidates.len() == election.elected_count {
        decision_log.add("All vacancies filled by candidates over quota");
        return ElectionResult {
            elected_candidates,
            quota,
            count_sheet: CountSheetCollection {
                count_sheets: vec![],
                aa_count_sheets: aa_round_sheets,
            },
            decision_log,
        };
    }

    /*
       3. If number of candidates elected in accordance with clause 2.4 of this Schedule does not fill all the
       vacancies, the preference votes of the elected candidates are then distributed among the
       remaining candidates in the following manner:
    */

    /*
        3.5. Repeat the steps in clauses 3.1 – 3.4 of this Schedule with the papers of the candidates with
        the second highest value of first preference papers, and so on in order of reducing number of
        points. If two surpluses are equal, the Returning Officer must decide which to take first
    */

    let mut aa_attempted = false;
    let mut count_sheet_entry_runoffs: Vec<CountSheet> = vec![];

    let mut removed_votes: Vec<uuid::Uuid> = vec![];

    let continuing_candidates =
        get_continuing_candidates(&elected_candidates, &defeated_candidates);

    let mut to_redistributed = elected_candidates.clone();

    // This assumes one person reached the quota that's bullshit
    loop {
        if to_redistributed.is_empty() {
            break;
        }

        let continuing_candidates =
            get_continuing_candidates(&elected_candidates, &defeated_candidates);

        /*
            3.1. Re-sort the first preference papers of the highest candidate according to the next preference
            shown for a continuing candidate (who is not yet recorded as elected or defeated), and count
            the total papers allotted to each continuing candidate. Note: Voting
            papers which show no further preference are called exhausted papers; record the number
            of them.
        */

        let top_score = active_cs.top_score(&to_redistributed);
        let mut highest_scoring_candidates = active_cs.matching_score(&to_redistributed, top_score);
        if highest_scoring_candidates.len() > 1 {
            decision_log.add("In 3.1 it does not specify how to break ties if two candidates have the same top score just picking alphabetically");
            highest_scoring_candidates.sort_by(|a, b| {
                election.candidates[*a]
                    .name
                    .cmp(&election.candidates[*b].name)
            });
        }

        let top_candidate = highest_scoring_candidates[0];
        to_redistributed.retain(|i| *i != top_candidate);

        decision_log.add(format!(
            "As per 3.1 found the top overall scoring candidate: $C{} with a score of {}",
            top_candidate, top_score,
        ));

        let saved_cs = active_cs.clone();

        distribute_surplus(
            &mut decision_log,
            &mut active_cs,
            quota,
            &continuing_candidates,
            top_candidate,
        );

        /*
            3.6 Subject to the Affirmative Action principles of clause 7 of this Schedule, any candidate who,
            whenever step in clause 3.4 of this Schedule is done, obtains a quota or more of points, is
            recorded as elected. No more papers are allotted to them beyond the bundle which gave the
            surplus. But all elected candidates who obtained their quota earlier than them, must have
            their surpluses dealt with first, even if theirs are larger.
        */
        let top_score = active_cs.top_score(&continuing_candidates);
        if top_score < quota {
            decision_log.add(format!(
                "As per 3.6 no candidates over quota with the top score: {} meaning nothing to do",
                top_score
            ));
            continue;
        }

        let mut candidates_with_top_score: Vec<_> =
            active_cs.matching_score(&continuing_candidates, top_score);
        decision_log.add(format!(
            "Top score: {} held by $C{:?}",
            top_score, candidates_with_top_score
        ));

        if candidates_with_top_score.len() > 1 {
            // As per 3.5 If two surpluses are equal, the Returning Officer must decide which to take first.
            decision_log.add("Multiple candidates with the same score selecting alphabetically");
            candidates_with_top_score.sort_by(|a, b| {
                election.candidates[*a]
                    .name
                    .cmp(&election.candidates[*b].name)
            });
        }

        let top_candidate = candidates_with_top_score[0];
        let just_elected = if !election.candidates[top_candidate].is_female
            && !aa_possible(&elected_candidates)
        {
            decision_log.add(format!(
                "As per 3.6 electing candidate $C{} would violate AA requirements resorting old count sheet",
                top_candidate
            ));
            if aa_attempted {
                decision_log
                    .add("However AA has already been attempted so electing the candidate anyway");
                vec![top_candidate]
            } else {
                active_cs = saved_cs;
                let elected = fill_aa_requirements(
                    &mut decision_log,
                    election,
                    &votes,
                    &elected_candidates,
                    &defeated_candidates,
                    &mut active_cs,
                    top_candidate,
                );
                aa_attempted = true;
                elected
            }
        } else {
            decision_log.add(format!(
                "As per 3.6 electing candidate $C{} would not violate AA requirements",
                top_candidate
            ));
            vec![top_candidate]
        };

        elected_candidates.extend(just_elected);

        if elected_candidates.len() >= election.elected_count {
            return ElectionResult {
                elected_candidates,
                quota,
                count_sheet: CountSheetCollection {
                    count_sheets: count_sheet_entry_runoffs,
                    aa_count_sheets: aa_round_sheets,
                },
                decision_log,
            };
        }
    }

    /*
        4. When the transfer of all surplus points of the elected candidates have been carried out or safely
        delayed, and vacancies remain to be filled, candidates then remaining are dealt with as follows:
    */

    /*
        4.1. Record as defeated all candidates with no points and remove their names from the counting
        table
    */
    // Reset since might get different results
    let mut aa_attempted = false;

    decision_log.add("As per 4.1 checking for candidates with no points");
    for candidate in continuing_candidates.clone() {
        if active_cs.get(candidate).score() == 0 {
            decision_log.add(format!(
                "Candidate $C{} has no points marking as defeated",
                candidate
            ));
            defeated_candidates.push(candidate);
        }
    }

    if elected_candidates.len()
        + get_continuing_candidates(&elected_candidates, &defeated_candidates).len()
        < election.elected_count
    {
        decision_log.add("Not enough continuing candidates to fill the vacancies the rules don't cover this picking defeated candidates until elected count is reached");
        // undefined outcome pick randomly from the defeated candidates

        while elected_candidates.len() < election.elected_count {
            let candidate = defeated_candidates.pop().unwrap();
            elected_candidates.push(candidate);
        }

        return ElectionResult {
            elected_candidates,
            quota,
            count_sheet: CountSheetCollection {
                count_sheets: count_sheet_entry_runoffs,
                aa_count_sheets: aa_round_sheets,
            },
            decision_log,
        };
    }

    loop {
        /*
            4.6. The procedure, of transferring the surpluses of successful candidates and of eliminating in
            succession the defeated candidates, is continued until the election is finished. The election is
            finished when:
        */
        let continuing_candidates =
            get_continuing_candidates(&elected_candidates, &defeated_candidates);

        // 4.6.1. The number of candidates recorded as elected equals the number of vacancies;or
        if elected_candidates.len() >= election.elected_count {
            decision_log.add("As per 4.6.1 the number of candidates recorded as elected equals the number of vacancies breaking");
            break;
        }

        /*
            4.6.2. Only one vacancy remains unfilled and two continuing candidates remain, in which case
            the candidate with the higher progress total is recorded as elected (since all the papers
            of the other candidate must go either to them or to exhausted); or
        */
        if election.elected_count - elected_candidates.len() == 1
            && continuing_candidates.len() == 2
        {
            decision_log.add("As per 4.6.2 only one vacancy remains unfilled and two continuing candidates remain");
            let a = active_cs.get(continuing_candidates[0]).score();
            let b = active_cs.get(continuing_candidates[1]).score();

            decision_log.add(format!(
                "Comparing candidates $C{} and $C{} with scores {} and {}",
                continuing_candidates[0], continuing_candidates[1], a, b
            ));

            let elected = if a > b {
                decision_log.add(format!(
                    "Candidate $C{} has a higher score marking as elected",
                    continuing_candidates[0]
                ));
                continuing_candidates[0]
            } else if a < b {
                decision_log.add(format!(
                    "Candidate $C{} has a higher score marking as elected",
                    continuing_candidates[1]
                ));
                continuing_candidates[1]
            } else {
                decision_log.add("Scores are equal selecting alphabetically Once again rules do not say what to do here");
                // Pick based on name
                let a_name = &election.candidates[continuing_candidates[0]].name;
                let b_name = &election.candidates[continuing_candidates[1]].name;
                if a_name < b_name {
                    continuing_candidates[0]
                } else {
                    continuing_candidates[1]
                }
            };

            if !election.candidates[elected].is_female && !aa_possible(&elected_candidates) {
                decision_log.add(format!(
                    "As per 4.6.2 electing candidate $C{} would violate AA requirements",
                    elected
                ));
                let aa_elected = fill_aa_requirements(
                    &mut decision_log,
                    election,
                    &votes,
                    &elected_candidates,
                    &defeated_candidates,
                    &mut active_cs,
                    elected,
                );
                elected_candidates.extend(aa_elected);
            } else {
                elected_candidates.push(elected);
            }
            break;
        }

        /*
            4.6.3. The number of continuing candidates is equal to the number of vacancies not yet filled,
            in which case all these candidates are recorded as elected.
        */
        if continuing_candidates.len() == election.elected_count - elected_candidates.len() {
            let mut continuing_candidates = continuing_candidates;
            decision_log.add("As per 4.6.3 the number of continuing candidates is equal to the number of vacancies not yet filled marking all as elected");

            // Fucking edge cases
            let female_count = elected_female_count(&elected_candidates) as i64;
            let females_needed = min_female_positions as i64 - female_count;

            if females_needed > 0 {
                let females_to_elect = continuing_candidates
                    .iter()
                    .filter(|i| election.candidates[**i].is_female)
                    .count() as i64;

                if females_to_elect >= females_needed {
                    break;
                }

                decision_log.add(format!(
                    "As per 4.6.3 electing candidates $C{:?} would violate AA requirements",
                    continuing_candidates
                ));

                continuing_candidates.sort_by(|a, b| {
                    active_cs
                        .get(*a)
                        .score()
                        .cmp(&active_cs.get(*b).score())
                        .reverse()
                });

                decision_log.add(format!(
                    "Sorted continuing candidates by score $C{:?}",
                    continuing_candidates,
                ));

                for candidate in continuing_candidates {
                    if !election.candidates[candidate].is_female
                        && !aa_possible(&elected_candidates)
                    {
                        decision_log.add(format!(
                            "As per 4.6.3 electing candidate $C{} would violate AA requirements",
                            candidate
                        ));
                        let aa_elected = fill_aa_requirements(
                            &mut decision_log,
                            election,
                            &votes,
                            &elected_candidates,
                            &defeated_candidates,
                            &mut active_cs,
                            candidate,
                        );
                        elected_candidates.extend(aa_elected);
                    } else {
                        decision_log.add(format!(
                            "As per 4.6.3 electing candidate $C{} would not violate AA requirements",
                            candidate
                        ));
                        elected_candidates.push(candidate);
                    }

                    if elected_candidates.len() >= election.elected_count {
                        break;
                    }
                }
            } else {
                decision_log.add(format!(
                    "As per 4.6.3 electing candidate $C{:?} would not violate AA requirements",
                    continuing_candidates
                ));
                elected_candidates.extend(continuing_candidates);
            }
            break;
        }

        /*
            4.2 Declare the candidate with the lowest progress total of points defeated and distribute their
            papers bundle by bundle in the order in which they were received. Each continuing candidate
            is allotted those papers, on which they are indicated as preferred to any other continuing
            candidate, at the same values at which the papers were originally received by the defeated
            candidate. Keep every bundle of papers separate even though a candidate has other papers
            of the same transfer value.
        */

        let continuing_candidates =
            get_continuing_candidates(&elected_candidates, &defeated_candidates);

        let lowest_score = active_cs.lowest_score(&continuing_candidates);
        let mut candidates_with_lowest_score =
            active_cs.matching_score(&continuing_candidates, lowest_score);

        decision_log.add(format!(
            "as per 4.2 the candidate with the lowest score {} total of points is removed with the candidates $C{:?}",
            lowest_score, candidates_with_lowest_score
        ));
        /*
            4.5. When it becomes necessary to eliminate a candidate and two (or more) candidates have
            equal progress totals lower than any other progress total, the Returning Officer shall decide
            by lot which candidate is to be eliminated first.
        */
        // as per other sections where is an equality of the value of first preference papers, the Returning Officer shall determine the order by lot.
        if candidates_with_lowest_score.len() > 1 {
            decision_log
                .add("as per 4.5 Multiple candidates with the same score selecting alphabetically");
            candidates_with_lowest_score.sort_by(|a, b| {
                election.candidates[*a]
                    .name
                    .cmp(&election.candidates[*b].name)
            });
        }

        let defeated_candidate = candidates_with_lowest_score[0];

        decision_log.add(format!(
            "Defeated candidate selected: $C{}",
            defeated_candidate
        ));

        defeated_candidates.push(defeated_candidate);

        let continuing_candidates =
            get_continuing_candidates(&elected_candidates, &defeated_candidates);

        // As per 4.2
        let papers_to_distribute = active_cs.get(defeated_candidate).papers.clone();

        decision_log.add(
            "As per 4.2 redistributing the defeated candidates papers at there value to the next preference",
        );

        for paper in papers_to_distribute {
            // As per 4.4 do not allocate more papers to the candidate who has reached the quota
            let cc_needing_votes = continuing_candidates
                .iter()
                .filter(|i| active_cs.get(**i).score() < quota)
                .cloned()
                .collect::<Vec<_>>();

            let mut transfer_tally = HashMap::new();
            if let Some(next_top) = paper.vote.top_continuing_preference(&cc_needing_votes) {
                // papers need to be kept separate
                *transfer_tally.entry(next_top).or_insert(0) += 1;
                active_cs.transfer_paper_4_2(defeated_candidate, next_top, &paper, paper.value);
            }

            for (candidate, count) in transfer_tally {
                decision_log.add(format!("Transferred {} votes to $C{}", count, candidate));
            }
        }

        decision_log.add("As Per 4.4 have transferred all of the defeated candidates papers before transferring a surplus");

        /*
            4.3. Clauses 3.6 to 3.7 of this Schedule apply to each bundle of voting papers dealt with in Clause
            4.2 of this Schedule.
        */

        let top_score = active_cs.top_score(&continuing_candidates);
        if top_score >= quota {
            let mut candidates_with_top_score: Vec<_> =
                active_cs.matching_score(&continuing_candidates, top_score);

            decision_log.add(format!(
                "as per 4.3 the top score {} held by $C{:?} is over the quota",
                top_score, candidates_with_top_score
            ));

            if candidates_with_top_score.len() > 1 {
                decision_log.add(
                    "As per 3.5 Multiple candidates with the same score selecting alphabetically",
                );
                // As per 3.5 If two surpluses are equal, the Returning Officer must decide which to take first.
                candidates_with_top_score.sort_by(|a, b| {
                    election.candidates[*a]
                        .name
                        .cmp(&election.candidates[*b].name)
                });
            }

            let top_candidate = candidates_with_top_score[0];

            decision_log.add(format!("Top candidate selected: $C{}", top_candidate));

            if !election.candidates[top_candidate].is_female
                && !aa_possible(&elected_candidates)
                && aa_attempted
            {
                decision_log
                    .add("Since AA has already been attempted electing the candidate anyway");
            }

            // Subject to the Affirmative Action principles of clause 7 of this Schedule
            let aa_happened = if !election.candidates[top_candidate].is_female
                && !aa_possible(&elected_candidates)
                && !aa_attempted
            {
                decision_log.add(
                    "Since electing this candidate would violate AA requirements following rules per 7.1",
                );
                aa_attempted = true;

                // Do AA here
                let elected = fill_aa_requirements(
                    &mut decision_log,
                    election,
                    &votes,
                    &elected_candidates,
                    &defeated_candidates,
                    &mut active_cs,
                    top_candidate,
                );
                elected_candidates.extend(elected);

                true
            } else {
                // Elect the candidate
                elected_candidates.push(top_candidate);
                decision_log.add(format!(
                    "Candidate $C{} elected with a score of {}",
                    top_candidate, top_score
                ));
                false
            };

            if elected_candidates.len() == election.elected_count {
                decision_log.add("All vacancies filled by candidates breaking");
                break;
            }

            // Since the distribution happens inside the AA function
            if aa_happened {
                decision_log.add("AA happened continuing");
                continue;
            }

            let elected_candidate = elected_candidates.last().unwrap();
            if top_score == quota {
                // Set aside the papers
                let run_off_entry = active_cs.get(*elected_candidate);
                for paper in &run_off_entry.papers {
                    removed_votes.push(paper.vote.created_by.clone());
                }
            } else {
                /*
                    4.4. When a candidate receives their quota in this way, no further papers are allotted to them,
                    and the distribution of the defeated candidate’s papers must be completed before the new
                    surplus is transferred. On the other hand, transfer of the new surplus must be either done or
                    safely delayed, before another candidate is eliminated
                */

                decision_log.add("As per 4.4 transferring the surplus of the elected candidate since all the surplus papers have been transferred");

                distribute_surplus(
                    &mut decision_log,
                    &mut active_cs,
                    quota,
                    &continuing_candidates,
                    *elected_candidate,
                );
            }
        } else {
            decision_log.add(format!(
                "as per 4.3 the top score {} is not over the quota looping",
                top_score
            ));
        }
    }

    ElectionResult {
        elected_candidates,
        quota,
        count_sheet: CountSheetCollection {
            count_sheets: count_sheet_entry_runoffs,
            aa_count_sheets: aa_round_sheets,
        },
        decision_log,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_results_everyone_wins() {
        let election = Election {
            candidates: vec![
                Candidate::new("AW", true),
                Candidate::new("BW", true),
                Candidate::new("CM", true),
            ],
            percent_female: 1.0,
            elected_count: 3,
        };

        let votes = vec![
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
        ];

        let result = get_result(&election, &votes);
        assert_eq!(result.elected_candidates, vec![0, 1, 2]);
    }

    #[test]
    fn test_get_result_e5_f50_wwwmm_mw() {
        let election = Election {
            candidates: vec![
                Candidate::new("AW", true),
                Candidate::new("BW", true),
                Candidate::new("CW", true),
                Candidate::new("DM", false),
                Candidate::new("EM", false),
                Candidate::new("FM", false),
                Candidate::new("GW", true),
            ],
            percent_female: 0.5,
            elected_count: 5,
        };

        let votes = vec![
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![1]),
            Vote::new_unknown(vec![1]),
            Vote::new_unknown(vec![1]),
            Vote::new_unknown(vec![1]),
            Vote::new_unknown(vec![1]),
            Vote::new_unknown(vec![1]),
            Vote::new_unknown(vec![2]),
            Vote::new_unknown(vec![2]),
            Vote::new_unknown(vec![2]),
            Vote::new_unknown(vec![2]),
            Vote::new_unknown(vec![2]),
            Vote::new_unknown(vec![3]),
            Vote::new_unknown(vec![3]),
            Vote::new_unknown(vec![3]),
            Vote::new_unknown(vec![3]),
            Vote::new_unknown(vec![4]),
            Vote::new_unknown(vec![4]),
            Vote::new_unknown(vec![4]),
            Vote::new_unknown(vec![4]),
            Vote::new_unknown(vec![5]),
        ];

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_get_results_e3_f50_wmm_section_3() {
        let election = Election {
            candidates: vec![
                Candidate::new("AW", true),
                Candidate::new("BM", false),
                Candidate::new("CM", false),
                Candidate::new("D", false),
                Candidate::new("E", false),
                Candidate::new("F", false),
                Candidate::new("G", false),
                Candidate::new("H", false),
                Candidate::new("I", false),
                Candidate::new("J", false),
            ],
            percent_female: 0.5,
            elected_count: 3,
        };

        // 0, 1 win by quota 2 wins by distribution
        let votes = vec![
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 2]),
            Vote::new_unknown(vec![0, 2]),
            Vote::new_unknown(vec![0, 2]),
            Vote::new_unknown(vec![1, 0, 2]),
            Vote::new_unknown(vec![1, 0, 2]),
            Vote::new_unknown(vec![1, 2]),
            Vote::new_unknown(vec![1, 2]),
            Vote::new_unknown(vec![1, 2]),
            Vote::new_unknown(vec![2, 1]),
            Vote::new_unknown(vec![3]),
            Vote::new_unknown(vec![4]),
            Vote::new_unknown(vec![5]),
            Vote::new_unknown(vec![6]),
        ];

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 2]);
    }

    #[test]
    fn test_get_results_e3_f50_wmm_section_4() {
        let election = Election {
            candidates: vec![
                Candidate::new("AW", true),
                Candidate::new("BM", false),
                Candidate::new("CM", false),
                Candidate::new("D", false),
                Candidate::new("E", false),
                Candidate::new("F", false),
                Candidate::new("G", false),
            ],
            percent_female: 0.5,
            elected_count: 3,
        };

        // 0, 1 win by quota 2 wins by distribution
        let votes = vec![
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 1]),
            Vote::new_unknown(vec![0, 1]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![2, 1]),
            Vote::new_unknown(vec![3, 2]),
            Vote::new_unknown(vec![4, 2]),
            Vote::new_unknown(vec![5]),
            Vote::new_unknown(vec![6]),
        ];

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 2]);
    }

    #[test]
    fn test_get_results_e3_f50_wmm_section_4_2() {
        let election = Election {
            candidates: vec![
                Candidate::new("AW", true),
                Candidate::new("BM", false),
                Candidate::new("CM", false),
                Candidate::new("D", false),
                Candidate::new("E", false),
                Candidate::new("F", false),
                Candidate::new("G", false),
            ],
            percent_female: 0.,
            elected_count: 4,
        };

        let votes = vec![
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 1]),
            Vote::new_unknown(vec![0, 1]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![2, 1]),
            Vote::new_unknown(vec![3, 2, 4]),
            Vote::new_unknown(vec![4, 2]),
            Vote::new_unknown(vec![5]),
            Vote::new_unknown(vec![6]),
        ];

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 2, 4]);
    }

    #[test]
    fn test_get_results_e4_section_4_no_votes() {
        let election = Election {
            candidates: vec![
                Candidate::new("AW", true),
                Candidate::new("BM", false),
                Candidate::new("CM", false),
                Candidate::new("D", false),
                Candidate::new("E", false),
                Candidate::new("F", false),
                Candidate::new("G", false),
            ],
            percent_female: 0.,
            elected_count: 4,
        };

        // 0, 1 win by quota 2 wins by distribution
        let votes = vec![
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 1]),
            Vote::new_unknown(vec![0, 1]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![2, 1]),
        ];

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 2, 6]);
    }

    #[test]
    fn test_get_results_e4_section_4_6_2() {
        let election = Election {
            candidates: vec![
                Candidate::new("AW", true),
                Candidate::new("BM", false),
                Candidate::new("CM", false),
                Candidate::new("D", false),
                Candidate::new("E", false),
                Candidate::new("G", false),
            ],
            percent_female: 0.,
            elected_count: 4,
        };

        // 0, 1 win by quota 2 wins by distribution
        let votes = vec![
            Vote::new_unknown(vec![0]),
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![1, 0, 2]),
            Vote::new_unknown(vec![1, 0, 2]),
            Vote::new_unknown(vec![1, 0, 2]),
            Vote::new_unknown(vec![1, 0, 2]),
            Vote::new_unknown(vec![2, 1]),
            Vote::new_unknown(vec![3, 4]),
            Vote::new_unknown(vec![4]),
            Vote::new_unknown(vec![4]),
            Vote::new_unknown(vec![5]),
            Vote::new_unknown(vec![5]),
        ];

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 2, 4]);
    }

    #[test]
    fn test_get_results_e4_section_4_6_3() {
        let election = Election {
            candidates: vec![
                Candidate::new("AW", true),
                Candidate::new("BM", false),
                Candidate::new("CM", false),
                Candidate::new("D", false),
                Candidate::new("G", false),
            ],
            percent_female: 0.,
            elected_count: 4,
        };

        // 0, 1 win by quota 2 wins by distribution
        let votes = vec![
            Vote::new_unknown(vec![0, 1, 2]),
            Vote::new_unknown(vec![0, 1]),
            Vote::new_unknown(vec![0, 1]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![1, 0]),
            Vote::new_unknown(vec![1, 0, 4]),
            Vote::new_unknown(vec![2, 1]),
            Vote::new_unknown(vec![3]),
            Vote::new_unknown(vec![4]),
        ];

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 2, 4]);
    }

    #[test]
    fn test_get_results_e4_section_4_4() {
        let election = Election {
            candidates: vec![
                Candidate::new("A", false),
                Candidate::new("B", false),
                Candidate::new("C", false),
                Candidate::new("D", false),
                Candidate::new("G", false),
                Candidate::new("H", false),
                Candidate::new("I", false),
                Candidate::new("J", false),
                Candidate::new("K", false),
                Candidate::new("L", false),
                Candidate::new("M", false),
                Candidate::new("O", false),
                Candidate::new("P", false),
                Candidate::new("Q", false),
            ],
            percent_female: 0.,
            elected_count: 6,
        };

        let mut votes = vec![];
        for _ in 0..10 {
            votes.push(Vote::new_unknown(vec![0, 4]));
            votes.push(Vote::new_unknown(vec![1, 5]));
        }
        for _ in 0..5 {
            votes.push(Vote::new_unknown(vec![2, 4]));
            votes.push(Vote::new_unknown(vec![3, 5]));
        }

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_get_results_e4_section_2_4_aa() {
        let election = Election {
            candidates: vec![
                Candidate::new("A", false),
                Candidate::new("B", false),
                Candidate::new("C", false),
                Candidate::new("D", true),
                Candidate::new("G", false),
            ],
            percent_female: 0.5,
            elected_count: 3,
        };

        let mut votes = vec![];
        for _ in 0..5 {
            votes.push(Vote::new_unknown(vec![0, 2, 3]));
            votes.push(Vote::new_unknown(vec![1, 2, 3]));
        }
        for _ in 0..4 {
            votes.push(Vote::new_unknown(vec![2]));
        }
        votes.push(Vote::new_unknown(vec![3]));

        let result = get_result(&election, &votes);

        assert_eq!(result.elected_candidates, vec![0, 1, 3]);
    }

    #[test]
    fn test_get_results_section_4_6_3_aa_fail() {
        let election = Election {
            candidates: vec![
                Candidate::new("A", false),
                Candidate::new("B", false),
                Candidate::new("C", false),
                Candidate::new("D", false),
                Candidate::new("E", true),
                Candidate::new("F", true),
                Candidate::new("G", true),
                Candidate::new("H", true),
                Candidate::new("I", true),
            ],
            percent_female: 0.5,
            elected_count: 5,
        };

        let mut votes = vec![];
        for _ in 0..20 {
            votes.push(Vote::new_unknown(vec![0, 2]));
        }
        for _ in 0..1000 {
            votes.push(Vote::new_unknown(vec![2, 1]));
        }
        votes.push(Vote::new_unknown(vec![5]));
        for _ in 0..2 {
            votes.push(Vote::new_unknown(vec![6]));
        }
        for _ in 0..50 {
            votes.push(Vote::new_unknown(vec![3]));
        }
        votes.push(Vote::new_unknown(vec![0]));

        let result = get_result(&election, &votes);
        assert_eq!(result.elected_candidates, vec![2, 1, 3, 0, 6]);
    }
}
