-- Your SQL goes here
CREATE TABLE api_keys (
    id UUID PRIMARY KEY,
    name text NOT NULL,
    hashed_key text NOT NULL,
    role role DEFAULT role::'player',
    created_at timestamptz DEFAULT now(),
    revoked_at timestamptz
);
