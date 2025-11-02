-- Your SQL goes here
CREATE TABLE server_allocations (
    id UUID PRIMARY KEY,
    match_id UUID FOREIGN KEY matches(id),
    server_id UUID FOREIGN KEY game_servers(id),
    allocated_at timestamptz NOT NULL,
    expires_at timestamptz,
    status server_alloc_status
);
