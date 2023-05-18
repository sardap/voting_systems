CREATE TABLE cumulative_elections (
    election_id UUID NOT NULL PRIMARY KEY REFERENCES elections(id),
    options TEXT[] NOT NULL,
    max_votes INTEGER NOT NULL
);

CREATE TABLE cumulative_votes (
    id UUID NOT NULL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    created_by UUID NOT NULL,
    votes INTEGER[] NOT NULL,
    UNIQUE (created_by, election_id)
);
