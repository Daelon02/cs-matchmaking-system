-- Your SQL goes here
CREATE TABLE queues_entries (
    id UUID PRIMARY KEY,
    queue_id UUID FOREIGN KEY queues(id),
    player_id UUID NULL FOREIGN KEY players(id),
    party_id UUID NULL FOREIGN KEY parties(id),
    mmr_snapshot int NOT NULL,
    latency_ms int,
    enqueued_at timestamptz DEFAULT now()
    status queue_status DEFAULT queue_status::'queued',
    priority int DEFAULT 0
);
