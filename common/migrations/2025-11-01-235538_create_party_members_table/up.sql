-- Your SQL goes here
CREATE TABLE party_members (
    party_id UUID FOREIGN KEY parties(id),
    player_id UUID FOREIGN KEY players(id),
    role party_role NOT NULL,
    joined_at timestamptz DEFAULT now()
);
