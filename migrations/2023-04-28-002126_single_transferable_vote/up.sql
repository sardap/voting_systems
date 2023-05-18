CREATE TABLE stv_elections (
    election_id UUID NOT NULL PRIMARY KEY REFERENCES elections(id),
    options TEXT[] NOT NULL,
    elected_count INTEGER NOT NULL
);

CREATE TABLE stv_votes (
    id UUID NOT NULL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    created_by UUID NOT NULL,
    votes INTEGER[] NOT NULL,
    UNIQUE (created_by, election_id)
);

CREATE TABLE borda_count_elections (
    election_id UUID NOT NULL PRIMARY KEY REFERENCES elections(id),
    options TEXT[] NOT NULL
);

CREATE TABLE borda_count_votes (
    id UUID NOT NULL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    created_by UUID NOT NULL,
    votes INTEGER[] NOT NULL,
    UNIQUE (created_by, election_id)
);
