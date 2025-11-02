-- Your SQL goes here
CREATE TABLE leaderboard_snapshots(
    id UUID PRIMARY KEY,
    mode text,
    taken_at timestamptz,
    board jsonb
);
