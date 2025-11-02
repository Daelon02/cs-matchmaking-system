// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "game_server_status"))]
    pub struct GameServerStatus;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "mode"))]
    pub struct Mode;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "party_inv_status"))]
    pub struct PartyInvStatus;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "party_role"))]
    pub struct PartyRole;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "penalty_type"))]
    pub struct PenaltyType;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "queue_status"))]
    pub struct QueueStatus;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rating_change_reason"))]
    pub struct RatingChangeReason;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "report_reason"))]
    pub struct ReportReason;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "server_alloc_status"))]
    pub struct ServerAllocStatus;

    #[derive(Debug, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "server_status"))]
    pub struct ServerStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    api_keys (id) {
        id -> Uuid,
        name -> Text,
        hashed_key -> Text,
        role -> Nullable<Role>,
        created_at -> Nullable<Timestamptz>,
        revoked_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GameServerStatus;

    game_servers (id) {
        id -> Uuid,
        region -> Text,
        host -> Inet,
        port -> Int4,
        capacity -> Int2,
        status -> GameServerStatus,
        last_heartbeat -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    leaderboard_snapshots (id) {
        id -> Uuid,
        mode -> Nullable<Text>,
        taken_at -> Nullable<Timestamptz>,
        board -> Nullable<Jsonb>,
    }
}

diesel::table! {
    match_players (match_id) {
        match_id -> Uuid,
        player_id -> Nullable<Uuid>,
        team -> Nullable<Int2>,
        mmr_at_start -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Mode;
    use super::sql_types::ServerStatus;

    matches (id) {
        id -> Uuid,
        queue_id -> Nullable<Uuid>,
        mode -> Mode,
        region -> Text,
        team_size -> Int2,
        status -> ServerStatus,
        created_at -> Timestamptz,
        ready_at -> Nullable<Timestamptz>,
        started_at -> Nullable<Timestamptz>,
        finished_at -> Nullable<Timestamptz>,
        server_ip -> Nullable<Inet>,
        server_port -> Nullable<Int4>,
        server_token -> Nullable<Text>,
        server_ttl -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    parties (id) {
        id -> Uuid,
        leader_id -> Nullable<Uuid>,
        medium_mmr -> Int4,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PartyInvStatus;

    party_invitations (id) {
        id -> Uuid,
        party_id -> Nullable<Uuid>,
        invited_player_id -> Nullable<Uuid>,
        inviter_id -> Nullable<Uuid>,
        status -> PartyInvStatus,
        expires_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PartyRole;

    party_members (party_id) {
        party_id -> Uuid,
        player_id -> Nullable<Uuid>,
        role -> PartyRole,
        joined_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PenaltyType;

    penalties (id) {
        id -> Uuid,
        player_id -> Nullable<Uuid>,
        #[sql_name = "type"]
        type_ -> Nullable<PenaltyType>,
        score -> Nullable<Int4>,
        expires_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    player_ratings (player_id) {
        player_id -> Uuid,
        mode -> Nullable<Text>,
        mmr -> Nullable<Int4>,
        last_update -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    player_sessions (id) {
        id -> Uuid,
        player_id -> Nullable<Uuid>,
        created -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        ip -> Text,
        is_active -> Nullable<Bool>,
    }
}

diesel::table! {
    players (id) {
        id -> Uuid,
        email -> Text,
        avatar_url -> Nullable<Text>,
        bio -> Nullable<Text>,
        region -> Nullable<Text>,
        nickname -> Nullable<Text>,
        first_ip -> Inet,
        created -> Nullable<Timestamptz>,
        updated -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Mode;

    queues (id) {
        id -> Uuid,
        code -> Nullable<Text>,
        mode -> Mode,
        team_size -> Int2,
        medium_mmr -> Int4,
        region -> Text,
        constraints -> Jsonb,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QueueStatus;

    queues_entries (id) {
        id -> Uuid,
        queue_id -> Nullable<Uuid>,
        player_id -> Nullable<Uuid>,
        party_id -> Nullable<Uuid>,
        mmr_snapshot -> Int4,
        latency_ms -> Nullable<Int4>,
        enqueued_at -> Nullable<Timestamptz>,
        status -> Nullable<QueueStatus>,
        priority -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RatingChangeReason;

    rating_changes (id) {
        id -> Uuid,
        player_id -> Nullable<Uuid>,
        match_id -> Nullable<Uuid>,
        mode -> Nullable<Text>,
        delta -> Nullable<Int4>,
        new_mmr -> Nullable<Text>,
        reason -> Nullable<RatingChangeReason>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ReportReason;

    reports (id) {
        id -> Uuid,
        reported_player_id -> Nullable<Uuid>,
        reporter_player_id -> Nullable<Uuid>,
        reason -> Nullable<ReportReason>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ServerAllocStatus;

    server_allocations (id) {
        id -> Uuid,
        match_id -> Nullable<Uuid>,
        server_id -> Nullable<Uuid>,
        allocated_at -> Timestamptz,
        expires_at -> Nullable<Timestamptz>,
        status -> Nullable<ServerAllocStatus>,
    }
}

diesel::joinable!(match_players -> matches (match_id));
diesel::joinable!(match_players -> players (player_id));
diesel::joinable!(matches -> queues (queue_id));
diesel::joinable!(parties -> players (leader_id));
diesel::joinable!(party_invitations -> parties (party_id));
diesel::joinable!(party_members -> parties (party_id));
diesel::joinable!(party_members -> players (player_id));
diesel::joinable!(penalties -> players (player_id));
diesel::joinable!(player_ratings -> players (player_id));
diesel::joinable!(player_sessions -> players (id));
diesel::joinable!(queues_entries -> parties (party_id));
diesel::joinable!(queues_entries -> players (player_id));
diesel::joinable!(queues_entries -> queues (queue_id));
diesel::joinable!(rating_changes -> matches (match_id));
diesel::joinable!(rating_changes -> players (player_id));
diesel::joinable!(server_allocations -> game_servers (server_id));
diesel::joinable!(server_allocations -> matches (match_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    game_servers,
    leaderboard_snapshots,
    match_players,
    matches,
    parties,
    party_invitations,
    party_members,
    penalties,
    player_ratings,
    player_sessions,
    players,
    queues,
    queues_entries,
    rating_changes,
    reports,
    server_allocations,
);
