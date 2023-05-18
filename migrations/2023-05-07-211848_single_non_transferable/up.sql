CREATE TABLE single_non_transferable_elections (
    election_id UUID NOT NULL PRIMARY KEY REFERENCES elections(id),
    options TEXT[] NOT NULL,
    elected_count INTEGER NOT NULL

);

CREATE TABLE single_non_transferable_votes (
    id UUID NOT NULL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    created_by UUID NOT NULL,
    votes BOOLEAN[] NOT NULL,
    UNIQUE (created_by, election_id)
);
