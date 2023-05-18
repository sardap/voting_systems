export enum ElectionType {
    PreferentialVoting = 'Preferential Voting',
    FirstPastThePost = 'First past the post',
    SingleTransferableVote = 'Single Transferable Vote',
    BordaCount = 'Borda Count',
    Approval = 'Approval',
    Star = 'Star',
    Cumulative = 'Cumulative',
    AntiPlurality = 'Anti Plurality',
    SingleParty = 'One Party',
    ThreeTwoOne = '3-2-1',
    CondorcetMethod = 'Condorcet winner or Ranked Pairs',
    MajorityJudgment = 'Majority Judgment',
    Score = 'Score',
    UsualJudgment = 'Usual Judgment',
    SingleNonTransferable = 'Single Non-Transferable Vote'
}

export function election_type_to_path(election_type: ElectionType) {
    switch (election_type) {
        case ElectionType.PreferentialVoting:
            return 'preferential_voting';
        case ElectionType.SingleTransferableVote:
            return 'single_transferable_vote';
        case ElectionType.BordaCount:
            return 'borda_count';
        case ElectionType.Approval:
            return 'approval';
        case ElectionType.Star:
            return 'star';
        case ElectionType.Cumulative:
            return 'cumulative';
        case ElectionType.AntiPlurality:
            return 'anti_plurality';
        case ElectionType.SingleParty:
            return 'single_party';
        case ElectionType.ThreeTwoOne:
            return 'three_two_one';
        case ElectionType.CondorcetMethod:
            return 'condorcet_method';
        case ElectionType.MajorityJudgment:
            return 'majority_judgment';
        case ElectionType.Score:
            return 'score';
        case ElectionType.UsualJudgment:
            return 'usual_judgment';
        case ElectionType.SingleNonTransferable:
            return 'single_non_transferable';
        case ElectionType.FirstPastThePost:
            return 'first_past_the_post';
    }
}

interface GetTokenResponse {
    token: string;
}

export async function get_token(api_key: string, election_id: string) {
    const params = new URLSearchParams({ api_key: api_key });

    const response = await fetch(`/api/v1/${election_id}/get_token?` + params.toString());
    const result: GetTokenResponse = await response.json();
    return result.token;
}

export async function change_extra(election_id: string, api_key: string, new_public?: boolean, new_voting_lock?: boolean) {
    const params = new URLSearchParams({ api_key: api_key });
    console.log(`Updating change extra value for election ${election_id}`);
    const response = await fetch(`/api/v1/${election_id}/change_extra?${params.toString()}`,
        {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                new_public: new_public,
                new_voting_locked: new_voting_lock
            })
        }
    );
    console.log(`Response: ${response.status} ${response.statusText}`);
}

export interface GetExtraResponse {
    voting_lock: boolean,
    locked: boolean,
}

export async function get_extra(election_id: string, api_key: string) {
    const params = new URLSearchParams({ api_key: api_key });
    const response = await fetch(`/api/v1/${election_id}/get_extra?${params.toString()}`);
    const result: GetExtraResponse = await response.json();
    return result;
}

export interface GenericElection {
    id: string;
    title: string;
    require_token: Boolean;
    options: string[];
}

export async function get_generic_election(election_type: ElectionType, election_id: string) {
    const response = await fetch(`/api/v1/${election_type_to_path(election_type)}/${election_id}`);
    const tmp: GenericElection = await response.json();
    return tmp;
}

export interface GenericVoteResult {
    created_by: string;
}

export interface GenericElectionResult {
    vote_count: number;
}

export async function get_generic_election_result(election_type: ElectionType, election_id: string, api_key?: string) {
    return await get_result<GenericElectionResult>(election_type, election_id, api_key);
}

export async function submit_generic_vote(election_type: ElectionType, election_id: string, vote_token: string | undefined, votes: number[] | boolean[]) {
    const params = new URLSearchParams();

    if (vote_token) {
        params.append('vote_token', vote_token);
    }

    const response = await fetch(
        `/api/v1/${election_type_to_path(election_type)}/${election_id}/new_vote?` + params.toString(),
        {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                votes: votes
            })
        }
    );

    return response;
}

export interface GenericElectionCreate {
    title: string;
    options: string[];
    require_token: Boolean;
}

export async function create_generic_election(election_type: ElectionType, api_key: string, arg: GenericElectionCreate) {
    const params = new URLSearchParams({ api_key: api_key });
    const url = `/api/v1/${election_type_to_path(election_type)}?` + params.toString();
    const response = await fetch(url,
        {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(arg)
        }
    );
    return response;
}

export interface RankedChoiceVoteTally {
    votes: number[],
    count: number,
}

export interface VoteLogEntry {
    created_by: string;
    votes: number[];
}

export interface ElectionResult {
    candidates: string[];
    votes: VoteLogEntry[];
}

export interface LogVotes {
    name: string;
    votes: number;
}

export interface LogEntry {
    eliminated: number[];
    votes: LogVotes[];
}

async function get_result<T>(election_type: ElectionType, election_id: string, api_key?: string): Promise<T> {
    const params = new URLSearchParams({});
    if (api_key) {
        params.append('api_key', api_key);
    }

    const response = await fetch(`/api/v1/${election_type_to_path(election_type)}/${election_id}/get_result?${params.toString()}`);
    return await response.json();
}

export interface PrefElectionResult {
    candidates: string[];
    log: LogEntry[];
    winner: number;
    votes: RankedChoiceVoteTally[];
    vote_count: number;
}

export async function get_preferential_voting_result(api_key: string, election_id: string): Promise<PrefElectionResult> {
    return await get_result<PrefElectionResult>(ElectionType.PreferentialVoting, election_id, api_key);
}

export interface ElectionBase {
    id: string;
    title: string;
    require_token: Boolean;
}


export interface PrefElection {
    id: string;
    title: string;
    require_token: Boolean,
    options: string[];
}

export interface CreateElection {
    title: string;
    require_token: Boolean;
}

export interface PrefCreateElection {
    title: string;
    options: string[];
    require_token: Boolean;
}

export interface CreateElectionResponse {
    id: string;
    key: string;
}

export interface StvCreateElection {
    title: string;
    options: string[];
    require_token: Boolean;
    elected_count: number;
}

export interface StvVote {
    created_by: string;
    votes: number[];
}

export interface StvRound {
    vote_counts: Record<number, number>;
    eliminated_candidates: number[];
    elected_candidates: number[];
}

export interface StvResult {
    candidates: string[];
    elected_candidates: number[];
    votes: RankedChoiceVoteTally[];
    rounds: StvRound[];
}


export async function get_single_transferable_vote_election_result(api_key: string, election_id: string, eliminated_candidates: number[]): Promise<StvResult> {
    const params = new URLSearchParams({ api_key: api_key });
    if (eliminated_candidates.length > 0) {
        params.append('pre_eliminated_candidates', eliminated_candidates.join(','));
    }

    const response = await fetch(`/api/v1/single_transferable_vote/${election_id}/get_result?` + params.toString());
    return await response.json();
}


export interface BordaCountTally {
    option_index: number;
    vote_count: number;
}

export interface BordaCountVote {
    created_by: string,
    votes: number[],
}

export interface BordaCountResult {
    options: string[];
    winner: number;
    vote_tally: BordaCountTally[],
    votes: BordaCountVote[],
    vote_count: number,
}

export async function get_borda_count_election_result(api_key: string, election_id: string): Promise<BordaCountResult> {
    return await get_result<BordaCountResult>(ElectionType.BordaCount, election_id, api_key);
}


export interface ApprovalTally {
    option_index: number;
    approval_count: number;
}

export interface ApprovalVote {
    created_by: string,
    votes: boolean[],
}

export interface ApprovalResult {
    options: string[];
    winner: number;
    approve_tally: ApprovalTally[],
    vote_count: number,
    votes: ApprovalVote[],
}

export async function get_approval_election_result(api_key: string, election_id: string): Promise<ApprovalResult> {
    return await get_result<ApprovalResult>(ElectionType.Approval, election_id, api_key);
}

export interface StarTally {
    option_index: number;
    points_count: number;
}

export interface StarRunoffScore {
    option_index: number;
    vote_count: number;
}

export interface StarVote {
    created_by: string,
    votes: number[],
}

export interface StarResult {
    options: string[];
    points_tally: StarTally[];
    runoff: StarRunoffScore[];
    winner: number;
    vote_count: number;
    votes: StarVote[];
}

export async function get_star_election_result(api_key: string, election_id: string): Promise<StarResult> {
    return await get_result<StarResult>(ElectionType.Star, election_id, api_key);
}

export interface CreateCumulativeElection extends GenericElectionCreate {
    max_votes: number;
}
export interface CumulativeVote {
    created_by: string;
    votes: number[];
}

export interface CumulativeTally {
    option_index: number;
    vote_count: number;
}

export interface CumulativeResult {
    options: string[];
    votes_tally: CumulativeTally[];
    winner: number;
    vote_count: number;
    votes: CumulativeVote[];
}

export async function get_cumulative_result(api_key: string, election_id: string): Promise<CumulativeResult> {
    return await get_result<CumulativeResult>(ElectionType.Cumulative, election_id, api_key);
}

export interface CumulativeElection extends GenericElection {
    max_votes: number;
}

export async function get_cumulative_election(election_id: string): Promise<CumulativeElection> {
    return await get_generic_election(ElectionType.Cumulative, election_id) as CumulativeElection;
}

export interface AntiPluralityTally {
    option_index: number;
    vote_count: number;
}

export interface AntiPluralityResult {
    options: string[];
    votes_tally: AntiPluralityTally[];
    winner: number;
    vote_count: number;
    votes: AntiPluralityVote[];
}

export interface AntiPluralityVote {
    created_by: string;
    vote: number;
}

export async function get_anti_plurality_result(api_key: string, election_id: string): Promise<AntiPluralityResult> {
    return await get_result<AntiPluralityResult>(ElectionType.AntiPlurality, election_id, api_key);
}

export async function submit_anti_plurality_vote(election_id: string, vote_token: string | undefined, vote: number) {
    const params = new URLSearchParams();

    if (vote_token) {
        params.append('vote_token', vote_token);
    }

    const response = await fetch(
        `/api/v1/${election_type_to_path(ElectionType.AntiPlurality)}/${election_id}/new_vote?` + params.toString(),
        {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                vote: vote
            })
        }
    );

    return response;
}


export async function submit_single_party_vote(election_id: string, vote_token: string | undefined, voted: boolean) {
    const params = new URLSearchParams();

    if (vote_token) {
        params.append('vote_token', vote_token);
    }

    const response = await fetch(
        `/api/v1/${election_type_to_path(ElectionType.SingleParty)}/${election_id}/new_vote?` + params.toString(),
        {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                voted: voted
            })
        }
    );

    return response;
}

export interface SinglePartyResult {
    options: string[];
    filled_votes: number;
    blank_votes: number;
    won: boolean;
    vote_count: number;
}

export async function get_single_party_result(api_key: string, election_id: string): Promise<SinglePartyResult> {
    return await get_result<SinglePartyResult>(ElectionType.SingleParty, election_id, api_key);
}

export enum GoodOkBad {
    Bad = 0,
    Ok = 1,
    Good = 2,
}

export function from_good_ok_bad_to_int(x: GoodOkBad): number {
    return x;
}

export interface ThreeTwoOneVote {
    created_by: string;
    votes: GoodOkBad[];
}

export interface ThreeTwoOneTally {
    option_index: number;
    good_count: number;
    ok_count: number;
    bad_count: number;
    score: number;
}

export interface ThreeTwoOneResult {
    options: string[];
    points_tally: ThreeTwoOneTally[];
    semifinalists: number[];
    finalists: number[];
    winner: number;
    vote_count: number;
    votes: ThreeTwoOneVote[];
}

export async function get_three_two_one_result(api_key: string, election_id: string): Promise<ThreeTwoOneResult> {
    return await get_result<ThreeTwoOneResult>(ElectionType.ThreeTwoOne, election_id, api_key);
}

export interface CondorcetMethodVote {
    created_by: string; // UUID represented as a string
    votes: number[];
}

export interface MatchedPair {
    runner: number;
    opponent: number;
    votes_for_runner: number;
    votes_for_opponent: number;
    difference: number;
}

export interface CondorcetMethodResult {
    options: string[];
    matchups: number[][];
    condorcet_winner: number | null;
    matched_pairs: MatchedPair[] | null;
    locked_in_pairwise_victories: number[][] | null;
    matched_pair_winner: number | null;
    last_resort_winner: number | null;
    votes: RankedChoiceVoteTally[];
    vote_count: number;
}

export async function get_condorcet_method_result(api_key: string, election_id: string): Promise<CondorcetMethodResult> {
    return await get_result<CondorcetMethodResult>(ElectionType.CondorcetMethod, election_id, api_key);
}

export enum MJRating {
    Terrible = 0,
    Poor = 1,
    Acceptable = 2,
    Good = 3,
    VeryGood = 4,
}

export function from_mj_rating_to_string(x: MJRating): string {
    switch (x) {
        case MJRating.Terrible:
            return "Terrible";
        case MJRating.Poor:
            return "Poor";
        case MJRating.Acceptable:
            return "Acceptable";
        case MJRating.Good:
            return "Good";
        case MJRating.VeryGood:
            return "Very Good";
    }

    return '';
}


export function from_mj_rating_to_int(x: MJRating): number {
    return x;
}

export interface MajorityJudgmentVote {
    created_by: String,
    votes: MJRating[],
}

export interface MajorityJudgmentTally {
    option_index: number,
    ratings: number[],
}

export interface MajorityJudgmentRunoff {
    modified_tally: MajorityJudgmentTally[],
    best_median: MJRating,
    participants: number[],
    winners: number[],
}

export interface MajorityJudgmentResult {
    options: string[],
    starting_tally: MajorityJudgmentTally[],
    runoff: MajorityJudgmentRunoff,
    score_result: ScoreResult | null,
    winner: number,
    vote_count: number,
    votes: MajorityJudgmentVote[],
}

export async function get_majority_judgment_result(api_key: string, election_id: string): Promise<MajorityJudgmentResult> {
    return await get_result<MajorityJudgmentResult>(ElectionType.MajorityJudgment, election_id, api_key);
}

export interface ScoreElection extends GenericElection {
    max_score: number;
}


export interface ElectionCreateScore extends GenericElectionCreate {
    max_score: number;
}

export interface ScoreVote {
    created_by: string
    votes: number[]
}

export interface ScoreRunoff {
    participants: number[],
    winners: number[],
    score_checked: number,
}

export interface ScoreTally {
    option_index: number,
    vote_count: number,
}

export interface ScoreResult {
    options: string[],
    runoff: ScoreRunoff | null,
    winner: number,
    vote_tally: ScoreTally[],
    votes: ScoreVote[],
    vote_count: number,
}

export async function get_score_result(api_key: string, election_id: string): Promise<ScoreResult> {
    return await get_result<ScoreResult>(ElectionType.Score, election_id, api_key);
}

export enum UJGrade {
    Bad = 0,
    Inadequate,
    Passable,
    Fair,
    Good,
    VeryGood,
    Excellent,
}

export function ujgrade_to_string(grade: UJGrade): string {
    switch (grade) {
        case UJGrade.Bad:
            return "Bad";
        case UJGrade.Inadequate:
            return "Inadequate";
        case UJGrade.Passable:
            return "Passable";
        case UJGrade.Fair:
            return "Fair";
        case UJGrade.Good:
            return "Good";
        case UJGrade.VeryGood:
            return "Very Good";
        case UJGrade.Excellent:
            return "Excellent";
    }
}

export function ujgrade_to_number(grade: UJGrade): number {
    return grade;
}

export interface UsualJudgmentTally {
    option_index: number;
    ratings: number[];
}

export interface Score {
    option_index: number;
    score: number;
}

export interface BreakTie {
    scores: Score[];
    winner: number;
}

export interface UsualJudgmentVote {
    created_by: string;
    votes: UJGrade[];
}

export interface UsualJudgmentResult {
    options: string[];
    starting_tally: UsualJudgmentTally[];
    best_grade: UJGrade;
    tie_info?: BreakTie;
    winner: number;
    vote_count: number;
    votes: UsualJudgmentVote[];
}

export interface SntvCreateElection {
    title: string;
    options: string[];
    require_token: Boolean;
    elected_count: number;
}

export interface SntvVote {
    created_by: string;
    votes: number[];
}

export interface SntvTally {
    option_index: number,
    vote_count: number,
}
export interface SntvResult {
    options: string[];
    winners: number[];
    vote_tally: SntvTally[];
    votes: SntvVote[];
    vote_count: number,
}
