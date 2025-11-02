-- Your SQL goes here
CREATE TABLE matches (
    id UUID PRIMARY KEY,
    queue_id UUID,
    mode mode NOT NULL,
    region text NOT NULL,
    team_size smallint NOT NULL,
    status server_status NOT NULL,
    created_at timestamptz NOT NULL,
    ready_at timestamptz NULL,
    started_at timestamptz NULL,
    finished_at timestamptz NULL,
    server_ip inet NULL,
    server_port int NULL,
    server_token text NULL,
    server_ttl timestamptz NULL,
    FOREIGN KEY (queue_id) REFERENCES queues(id)
);
