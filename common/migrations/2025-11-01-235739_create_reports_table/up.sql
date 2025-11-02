-- Your SQL goes here
CREATE TABLE reports(
    id UUID PRIMARY KEY,
    reported_player_id UUID FOREIGN KEY players(id),
    reporter_player_id UUID FOREIGN KEY players(id),
    reason report_reason,
    created_at timestamptz
);
