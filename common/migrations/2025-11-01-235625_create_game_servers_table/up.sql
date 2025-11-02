-- Your SQL goes here
CREATE TABLE game_servers (
    id UUID PRIMARY KEY,
    region text NOT NULL,
    host inet NOT NULL,
    port int NOT NULL,
    capacity smallint NOT NULL,
    status game_server_status NOT NULL,
    last_heartbeat timestamptz
);
