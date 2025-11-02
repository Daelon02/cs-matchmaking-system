-- Your SQL goes here
CREATE TABLE player_ratings (
    player_id UUID PRIMARY KEY,
    mode text,
    mmr int,
    last_update timestamptz,
    FOREIGN KEY (player_id) REFERENCES players(id)
);
