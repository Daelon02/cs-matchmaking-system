-- Your SQL goes here
CREATE TABLE parties (
    id UUID PRIMARY KEY,
    leader_id UUID,
    medium_mmr INT NOT NULL,
    created_at timestamptz DEFAULT now(),
    FOREIGN KEY (leader_id) REFERENCES players(id)
);
