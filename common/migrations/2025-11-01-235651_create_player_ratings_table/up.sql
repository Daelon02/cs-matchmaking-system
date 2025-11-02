-- Your SQL goes here
CREATE TABLE player_ratings (
    player_id UUID PRIMARY KEY FOREIGN KEY players(id),
    mode text,
    mmr int,
    last_update timestamptz
);
