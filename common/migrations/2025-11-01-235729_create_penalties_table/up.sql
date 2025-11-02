-- Your SQL goes here
CREATE TABLE penalties(
    id UUID PRIMARY KEY,
    player_id UUID FOREIGN KEY players(id),
    type penalty_type,
    score int,
    expires_at timestamptz,
    created_at timestamptz DEFAULT now()
);
