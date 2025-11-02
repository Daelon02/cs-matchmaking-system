-- Your SQL goes here
CREATE TYPE role AS ENUM ('server', 'admin', 'player', 'vip');
CREATE TYPE mode AS ENUM ('ranked', 'practice', 'custom');
CREATE TYPE queue_status AS ENUM ('queued', 'matched', 'cancelled', 'expired');
CREATE TYPE party_role AS ENUM ('leader', 'member');
CREATE TYPE party_inv_status AS ENUM ('pending','accepted','declined','expired');
CREATE TYPE server_status AS ENUM ('forming','ready','allocated','started','finished','failed');
CREATE TYPE game_server_status AS ENUM ('idle','allocating','busy','down');
CREATE TYPE server_alloc_status AS ENUM ('ok','failed','released');
CREATE TYPE rating_change_reason AS ENUM ('win', 'loss', 'decay', 'placement');
CREATE TYPE penalty_type AS ENUM ('leaver','afk','abuse', 'cheating');
CREATE TYPE report_reason AS ENUM ('smurfing', 'cheating', 'ghosting', 'abuse');