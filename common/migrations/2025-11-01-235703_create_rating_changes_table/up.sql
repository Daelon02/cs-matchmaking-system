-- Your SQL goes here
CREATE TABLE rating_changes (
    id UUID PRIMARY KEY,
    player_id UUID FOREIGN KEY players(id),
    match_id UUID FOREIGN KEY matches(id),
    mode text,
    delta int,
    new_mmr text,
    reason rating_change_reason,
    created_at timestamptz
);
