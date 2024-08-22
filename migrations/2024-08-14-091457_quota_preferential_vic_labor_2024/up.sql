CREATE TABLE quota_preferential_vic_labor_2024_elections (
    election_id UUID NOT NULL PRIMARY KEY REFERENCES elections(id),
    elected_count INTEGER NOT NULL
);

CREATE TABLE quota_preferential_vic_labor_2024_candidate (
    id BIGSERIAL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    candidate_name TEXT NOT NULL,
    is_female BOOLEAN NOT NULL
);

CREATE TABLE quota_preferential_vic_labor_2024_transferable_votes (
    id UUID NOT NULL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    created_by UUID NOT NULL,
    votes BIGINT[] NOT NULL,
    UNIQUE (created_by, election_id)
);
