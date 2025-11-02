-- Your SQL goes here
CREATE TABLE parties (
    id UUID PRIMARY KEY,
    leader_id UUID FOREIGN KEY players(id),
    medium_mmr INT NOT NULL,
    created_at timestamptz DEFAULT now()
);
