-- Your SQL goes here
CREATE TABLE party_invitations (
    id UUID PRIMARY KEY,
    party_id UUID,
    invited_player_id UUID,
    inviter_id UUID,
    status party_inv_status NOT NULL,
    expires_at timestamptz,
    FOREIGN KEY (party_id) REFERENCES parties(id),
    FOREIGN KEY (invited_player_id) REFERENCES players(id),
    FOREIGN KEY (inviter_id) REFERENCES players(id)
);
