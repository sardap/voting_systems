CREATE TABLE single_party_elections (
    election_id UUID NOT NULL PRIMARY KEY REFERENCES elections(id),
    options TEXT[] NOT NULL
);

CREATE TABLE single_party_votes (
    id UUID NOT NULL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    created_by UUID NOT NULL,
    voted BOOLEAN NOT NULL,
    UNIQUE (created_by, election_id)
);
