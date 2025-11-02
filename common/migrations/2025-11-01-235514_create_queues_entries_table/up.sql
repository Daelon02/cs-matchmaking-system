-- Your SQL goes here
CREATE TABLE queues_entries (
    id UUID PRIMARY KEY,
    queue_id UUID,
    player_id UUID NULL,
    party_id UUID NULL,
    mmr_snapshot int NOT NULL,
    latency_ms int,
    enqueued_at timestamptz DEFAULT now(),
    status queue_status DEFAULT 'queued',
    priority int DEFAULT 0,
    FOREIGN KEY (queue_id) REFERENCES queues(id),
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (party_id) REFERENCES parties(id)
);
