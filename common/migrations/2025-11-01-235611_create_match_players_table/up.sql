-- Your SQL goes here
CREATE TABLE match_players (
    match_id UUID PRIMARY KEY,
    player_id UUID,
    team smallint,
    mmr_at_start int,
    FOREIGN KEY (match_id) REFERENCES matches(id),
    FOREIGN KEY (player_id) REFERENCES players(id)
);
