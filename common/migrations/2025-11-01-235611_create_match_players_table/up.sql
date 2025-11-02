-- Your SQL goes here
CREATE TABLE match_players (
    match_id UUID FOREIGN KEY matches(id),
    player_id UUID FOREIGN KEY players(id),
    team smallint,
    mmr_at_start numeric(8,2)
);
