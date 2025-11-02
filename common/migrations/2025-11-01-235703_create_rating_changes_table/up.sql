-- Your SQL goes here
CREATE TABLE rating_changes (
    id UUID PRIMARY KEY,
    player_id UUID,
    match_id UUID,
    mode text,
    delta int,
    new_mmr text,
    reason rating_change_reason,
    created_at timestamptz,
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (match_id) REFERENCES matches(id)
);
