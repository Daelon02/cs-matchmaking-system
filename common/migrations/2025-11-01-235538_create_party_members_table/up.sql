-- Your SQL goes here
CREATE TABLE party_members (
    party_id UUID PRIMARY KEY,
    player_id UUID,
    role party_role NOT NULL,
    joined_at timestamptz DEFAULT now(),
    FOREIGN KEY (party_id) REFERENCES parties(id),
    FOREIGN KEY (player_id) REFERENCES players(id)
);
