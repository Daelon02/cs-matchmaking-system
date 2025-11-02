-- Your SQL goes here
CREATE TABLE server_allocations (
    id UUID PRIMARY KEY,
    match_id UUID,
    server_id UUID,
    allocated_at timestamptz NOT NULL,
    expires_at timestamptz,
    status server_alloc_status,
    FOREIGN KEY (match_id) REFERENCES matches(id),
    FOREIGN KEY (server_id) REFERENCES game_servers(id)
);
