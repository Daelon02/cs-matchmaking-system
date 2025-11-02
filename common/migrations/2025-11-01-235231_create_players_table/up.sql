-- Your SQL goes here
CREATE TABLE players (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    avatar_url TEXT,
    bio TEXT,
    region TEXT,
    nickname TEXT,
    first_ip inet NOT NULL,
    created timestampz DEFAULT now(),
    updated timestampz
);
