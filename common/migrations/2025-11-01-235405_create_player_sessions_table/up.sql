-- Your SQL goes here
CREATE TABLE player_sessions (
    id UUID PRIMARY KEY,
    player_id UUID FOREIGN KEY players(id),
    created timestamptz DEFAULT now(),
    expires_at timestamptz,
    ip TEXT NOT NULL,
    is_active bool DEFAULT true,
);
