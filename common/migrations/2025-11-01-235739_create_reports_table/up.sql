-- Your SQL goes here
CREATE TABLE reports(
    id UUID PRIMARY KEY,
    reported_player_id UUID,
    reporter_player_id UUID,
    reason report_reason,
    created_at timestamptz,
    FOREIGN KEY (reported_player_id) REFERENCES players(id),
    FOREIGN KEY (reporter_player_id) REFERENCES players(id)
);
