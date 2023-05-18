CREATE TABLE elections (
    id UUID NOT NULL PRIMARY KEY,
    requires_token BOOLEAN NOT NULL,
    title TEXT NOT NULL,
    manage_token UUID NOT NULL,
    created_time TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE voting_tokens (
    election_id UUID NOT NULL REFERENCES elections(id),
    token UUID NOT NULL,
    PRIMARY KEY(election_id, token)
);

CREATE TABLE preferential_elections (
    election_id UUID NOT NULL PRIMARY KEY REFERENCES elections(id),
    options TEXT[] NOT NULL
);

CREATE TABLE preferential_votes (
    id UUID NOT NULL PRIMARY KEY,
    election_id UUID NOT NULL REFERENCES elections(id),
    created_by UUID NOT NULL,
    votes INTEGER[] NOT NULL,
    UNIQUE (created_by, election_id)
);