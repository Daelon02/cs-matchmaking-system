use chrono::{DateTime, Utc};
use diesel::prelude::*;
use ipnetwork::IpNetwork;
use serde_json::Value;
use uuid::Uuid;

use crate::schema::*;
use crate::sql_types::types::{
    GameServerStatus, Mode, PartyInvStatus, PartyRole, PenaltyType, QueueStatus,
    RatingChangeReason, ReportReason, Role, ServerAllocStatus, ServerStatus,
};

//
// api_keys
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    pub id: Uuid,
    pub name: String,
    pub hashed_key: String,
    pub role: Option<Role>,
    pub created_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey<'a> {
    pub name: &'a str,
    pub hashed_key: &'a str,
    pub role: Option<Role>,
}

//
// game_servers
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = game_servers)]
pub struct GameServer {
    pub id: Uuid,
    pub region: String,
    pub host: IpNetwork,
    pub port: i32,
    pub capacity: i16,
    pub status: GameServerStatus,
    pub last_heartbeat: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = game_servers)]
pub struct NewGameServer {
    pub region: String,
    pub host: IpNetwork,
    pub port: i32,
    pub capacity: i16,
    pub status: GameServerStatus,
}

//
// leaderboard_snapshots
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = leaderboard_snapshots)]
pub struct LeaderboardSnapshot {
    pub id: Uuid,
    pub mode: Option<String>,
    pub taken_at: Option<DateTime<Utc>>,
    pub board: Option<Value>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = leaderboard_snapshots)]
pub struct NewLeaderboardSnapshot {
    pub mode: Option<String>,
    pub taken_at: Option<DateTime<Utc>>,
    pub board: Option<Value>,
}

//
// match_players  (без Identifiable через неоднозначний PK)
//
#[derive(Debug, Clone, Queryable, Selectable, Associations)]
#[diesel(table_name = match_players)]
#[diesel(belongs_to(Match, foreign_key = match_id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
pub struct MatchPlayer {
    pub match_id: Uuid,
    pub player_id: Option<Uuid>,
    pub team: Option<i16>,
    pub mmr_at_start: Option<i32>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = match_players)]
pub struct NewMatchPlayer {
    pub match_id: Uuid,
    pub player_id: Option<Uuid>,
    pub team: Option<i16>,
    pub mmr_at_start: Option<i32>,
}

//
// matches
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = matches)]
#[diesel(belongs_to(Queue, foreign_key = queue_id))]
pub struct Match {
    pub id: Uuid,
    pub queue_id: Option<Uuid>,
    pub mode: Mode,
    pub region: String,
    pub team_size: i16,
    pub status: ServerStatus,
    pub created_at: DateTime<Utc>,
    pub ready_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub server_ip: Option<IpNetwork>,
    pub server_port: Option<i32>,
    pub server_token: Option<String>,
    pub server_ttl: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = matches)]
pub struct NewMatch {
    pub queue_id: Option<Uuid>,
    pub mode: Mode,
    pub region: String,
    pub team_size: i16,
    pub status: ServerStatus,
}

//
// parties
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = parties)]
#[diesel(belongs_to(Player, foreign_key = leader_id))]
pub struct Party {
    pub id: Uuid,
    pub leader_id: Option<Uuid>,
    pub medium_mmr: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = parties)]
pub struct NewParty {
    pub leader_id: Option<Uuid>,
    pub medium_mmr: i32,
}

//
// party_invitations
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = party_invitations)]
#[diesel(belongs_to(Party, foreign_key = party_id))]
#[diesel(belongs_to(Player, foreign_key = invited_player_id))]
#[diesel(belongs_to(Player, foreign_key = inviter_id))]
pub struct PartyInvitation {
    pub id: Uuid,
    pub party_id: Option<Uuid>,
    pub invited_player_id: Option<Uuid>,
    pub inviter_id: Option<Uuid>,
    pub status: PartyInvStatus,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = party_invitations)]
pub struct NewPartyInvitation {
    pub party_id: Option<Uuid>,
    pub invited_player_id: Option<Uuid>,
    pub inviter_id: Option<Uuid>,
    pub status: PartyInvStatus,
    pub expires_at: Option<DateTime<Utc>>,
}

//
// party_members  (без Identifiable через неоднозначний PK)
//
#[derive(Debug, Clone, Queryable, Selectable, Associations)]
#[diesel(table_name = party_members)]
#[diesel(belongs_to(Party, foreign_key = party_id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
pub struct PartyMember {
    pub party_id: Uuid,
    pub player_id: Option<Uuid>,
    pub role: PartyRole,
    pub joined_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = party_members)]
pub struct NewPartyMember {
    pub party_id: Uuid,
    pub player_id: Option<Uuid>,
    pub role: PartyRole,
}

//
// penalties
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = penalties)]
#[diesel(belongs_to(Player, foreign_key = player_id))]
pub struct Penalty {
    pub id: Uuid,
    pub player_id: Option<Uuid>,
    #[diesel(column_name = type_)]
    pub r#type: Option<PenaltyType>,
    pub score: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = penalties)]
pub struct NewPenalty {
    pub player_id: Option<Uuid>,
    #[diesel(column_name = type_)]
    pub r#type: Option<PenaltyType>,
    pub score: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
}

//
// player_ratings
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = player_ratings)]
#[diesel(primary_key(player_id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
pub struct PlayerRating {
    pub player_id: Uuid,
    pub mode: Option<String>,
    pub mmr: Option<i32>,
    pub last_update: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = player_ratings)]
pub struct NewPlayerRating {
    pub player_id: Uuid,
    pub mode: Option<String>,
    pub mmr: Option<i32>,
}

//
// player_sessions
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = player_sessions)]
#[diesel(belongs_to(Player, foreign_key = player_id))]
pub struct PlayerSession {
    pub id: Uuid,
    pub player_id: Option<Uuid>,
    pub created: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub ip: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = player_sessions)]
pub struct NewPlayerSession<'a> {
    pub player_id: Option<Uuid>,
    pub ip: &'a str,
    pub is_active: Option<bool>,
    pub expires_at: Option<DateTime<Utc>>,
}

//
// players
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = players)]
pub struct Player {
    pub id: Uuid,
    pub email: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub region: Option<String>,
    pub nickname: Option<String>,
    pub first_ip: IpNetwork,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = players)]
pub struct NewPlayer<'a> {
    pub email: &'a str,
    pub avatar_url: Option<&'a str>,
    pub bio: Option<&'a str>,
    pub region: Option<&'a str>,
    pub nickname: Option<&'a str>,
    pub first_ip: IpNetwork,
}

//
// queues
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = queues)]
pub struct Queue {
    pub id: Uuid,
    pub code: Option<String>,
    pub mode: Mode,
    pub team_size: i16,
    pub medium_mmr: i32,
    pub region: String,
    pub constraints: Value,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = queues)]
pub struct NewQueue {
    pub code: Option<String>,
    pub mode: Mode,
    pub team_size: i16,
    pub medium_mmr: i32,
    pub region: String,
    pub constraints: Value,
}

//
// queues_entries
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = queues_entries)]
#[diesel(belongs_to(Queue, foreign_key = queue_id))]
#[diesel(belongs_to(Player, foreign_key = player_id))]
#[diesel(belongs_to(Party,  foreign_key = party_id))]
pub struct QueueEntry {
    pub id: Uuid,
    pub queue_id: Option<Uuid>,
    pub player_id: Option<Uuid>,
    pub party_id: Option<Uuid>,
    pub mmr_snapshot: i32,
    pub latency_ms: Option<i32>,
    pub enqueued_at: Option<DateTime<Utc>>,
    pub status: Option<QueueStatus>,
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = queues_entries)]
pub struct NewQueueEntry {
    pub queue_id: Option<Uuid>,
    pub player_id: Option<Uuid>,
    pub party_id: Option<Uuid>,
    pub mmr_snapshot: i32,
    pub latency_ms: Option<i32>,
    pub status: Option<QueueStatus>,
    pub priority: Option<i32>,
}

//
// rating_changes
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = rating_changes)]
#[diesel(belongs_to(Player, foreign_key = player_id))]
#[diesel(belongs_to(Match,  foreign_key = match_id))]
pub struct RatingChange {
    pub id: Uuid,
    pub player_id: Option<Uuid>,
    pub match_id: Option<Uuid>,
    pub mode: Option<String>,
    pub delta: Option<i32>,
    pub new_mmr: Option<String>, // за схемою: Text
    pub reason: Option<RatingChangeReason>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = rating_changes)]
pub struct NewRatingChange {
    pub player_id: Option<Uuid>,
    pub match_id: Option<Uuid>,
    pub mode: Option<String>,
    pub delta: Option<i32>,
    pub new_mmr: Option<String>,
    pub reason: Option<RatingChangeReason>,
}

//
// reports
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = reports)]
#[diesel(belongs_to(Player, foreign_key = reported_player_id))]
#[diesel(belongs_to(Player, foreign_key = reporter_player_id))]
pub struct Report {
    pub id: Uuid,
    pub reported_player_id: Option<Uuid>,
    pub reporter_player_id: Option<Uuid>,
    pub reason: Option<ReportReason>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = reports)]
pub struct NewReport {
    pub reported_player_id: Option<Uuid>,
    pub reporter_player_id: Option<Uuid>,
    pub reason: Option<ReportReason>,
}

//
// server_allocations
//
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = server_allocations)]
#[diesel(belongs_to(Match,      foreign_key = match_id))]
#[diesel(belongs_to(GameServer, foreign_key = server_id))]
pub struct ServerAllocation {
    pub id: Uuid,
    pub match_id: Option<Uuid>,
    pub server_id: Option<Uuid>,
    pub allocated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: Option<ServerAllocStatus>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = server_allocations)]
pub struct NewServerAllocation {
    pub match_id: Option<Uuid>,
    pub server_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: Option<ServerAllocStatus>,
}
