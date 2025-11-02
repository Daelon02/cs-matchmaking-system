-- Your SQL goes here
CREATE TABLE queues (
    id UUID PRIMARY KEY,
    code text UNIQUE,
    mode mode NOT NULL,
    team_size smallint NOT NULL,
    medium_mmr INT NOT NULL,
    region text NOT NULL,
    constraints jsonb NOT NULL,
    created_at timestamptz DEFAULT now()
);
