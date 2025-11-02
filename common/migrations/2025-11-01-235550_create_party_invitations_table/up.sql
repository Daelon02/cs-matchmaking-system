-- Your SQL goes here
CREATE TABLE party_invitations (
    id UUID PRIMARY KEY,
    party_id UUID FOREIGN KEY parties(id),
    invited_player_id UUID FOREIGN KEY players(id),
    inviter_id UUID FOREIGN KEY players(id),
    status party_inv_status NOT NULL,
    expires_at timestamptz
);
