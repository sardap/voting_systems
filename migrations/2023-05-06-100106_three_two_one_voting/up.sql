CREATE TABLE three_two_one_elections (
    election_id UUID NOT NULL PRIMARY KEY REFERENCES elections(id),
    options TEXT[] NOT NULL
);

CREATE TABLE three_two_one_votes (
    id UUID NOT NULL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    created_by UUID NOT NULL,
    votes INTEGER[] NOT NULL,
    UNIQUE (created_by, election_id)
);
