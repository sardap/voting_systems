// @generated automatically by Diesel CLI.

diesel::table! {
    anti_plurality_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    anti_plurality_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        vote -> Int4,
    }
}

diesel::table! {
    approval_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    approval_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Bool>>,
    }
}

diesel::table! {
    borda_count_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    borda_count_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    condorcet_method_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    condorcet_method_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    cumulative_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
        max_votes -> Int4,
    }
}

diesel::table! {
    cumulative_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    elections (id) {
        id -> Uuid,
        requires_token -> Bool,
        title -> Text,
        manage_token -> Uuid,
        created_time -> Timestamp,
        public -> Bool,
        voting_locked -> Bool,
    }
}

diesel::table! {
    majority_judgment_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    majority_judgment_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    preferential_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    preferential_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    score_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
        max_score -> Int4,
    }
}

diesel::table! {
    score_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    single_non_transferable_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
        elected_count -> Int4,
    }
}

diesel::table! {
    single_non_transferable_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Bool>>,
    }
}

diesel::table! {
    single_party_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    single_party_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        voted -> Bool,
    }
}

diesel::table! {
    star_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    star_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    stv_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
        elected_count -> Int4,
    }
}

diesel::table! {
    stv_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    three_two_one_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    three_two_one_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    usual_judgment_elections (election_id) {
        election_id -> Uuid,
        options -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    usual_judgment_votes (id) {
        id -> Uuid,
        election_id -> Uuid,
        created_by -> Uuid,
        votes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    voting_tokens (election_id, token) {
        election_id -> Uuid,
        token -> Uuid,
    }
}

diesel::joinable!(anti_plurality_elections -> elections (election_id));
diesel::joinable!(anti_plurality_votes -> elections (election_id));
diesel::joinable!(approval_elections -> elections (election_id));
diesel::joinable!(approval_votes -> elections (election_id));
diesel::joinable!(borda_count_elections -> elections (election_id));
diesel::joinable!(borda_count_votes -> elections (election_id));
diesel::joinable!(condorcet_method_elections -> elections (election_id));
diesel::joinable!(condorcet_method_votes -> elections (election_id));
diesel::joinable!(cumulative_elections -> elections (election_id));
diesel::joinable!(cumulative_votes -> elections (election_id));
diesel::joinable!(majority_judgment_elections -> elections (election_id));
diesel::joinable!(majority_judgment_votes -> elections (election_id));
diesel::joinable!(preferential_elections -> elections (election_id));
diesel::joinable!(preferential_votes -> elections (election_id));
diesel::joinable!(score_elections -> elections (election_id));
diesel::joinable!(score_votes -> elections (election_id));
diesel::joinable!(single_non_transferable_elections -> elections (election_id));
diesel::joinable!(single_non_transferable_votes -> elections (election_id));
diesel::joinable!(single_party_elections -> elections (election_id));
diesel::joinable!(single_party_votes -> elections (election_id));
diesel::joinable!(star_elections -> elections (election_id));
diesel::joinable!(star_votes -> elections (election_id));
diesel::joinable!(stv_elections -> elections (election_id));
diesel::joinable!(stv_votes -> elections (election_id));
diesel::joinable!(three_two_one_elections -> elections (election_id));
diesel::joinable!(three_two_one_votes -> elections (election_id));
diesel::joinable!(usual_judgment_elections -> elections (election_id));
diesel::joinable!(usual_judgment_votes -> elections (election_id));
diesel::joinable!(voting_tokens -> elections (election_id));

diesel::allow_tables_to_appear_in_same_query!(
    anti_plurality_elections,
    anti_plurality_votes,
    approval_elections,
    approval_votes,
    borda_count_elections,
    borda_count_votes,
    condorcet_method_elections,
    condorcet_method_votes,
    cumulative_elections,
    cumulative_votes,
    elections,
    majority_judgment_elections,
    majority_judgment_votes,
    preferential_elections,
    preferential_votes,
    score_elections,
    score_votes,
    single_non_transferable_elections,
    single_non_transferable_votes,
    single_party_elections,
    single_party_votes,
    star_elections,
    star_votes,
    stv_elections,
    stv_votes,
    three_two_one_elections,
    three_two_one_votes,
    usual_judgment_elections,
    usual_judgment_votes,
    voting_tokens,
);
