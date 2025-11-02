use diesel::query_builder::QueryId;
use diesel::sql_types::*;
use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::Role")]
pub enum Role {
    Server,
    Admin,
    Vip,
    Player,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::Mode")]
pub enum Mode {
    Ranked,
    Practice,
    Custom,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::QueueStatus")]
pub enum QueueStatus {
    Queued,
    Matched,
    Cancelled,
    Expired,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::PartyRole")]
pub enum PartyRole {
    Leader,
    Member,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::PartyInvStatus")]
pub enum PartyInvStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::ServerStatus")]
pub enum ServerStatus {
    Forming,
    Ready,
    Allocated,
    Started,
    Finished,
    Failed,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::GameServerStatus")]
pub enum GameServerStatus {
    Idle,
    Allocating,
    Busy,
    Down,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::ServerAllocStatus")]
pub enum ServerAllocStatus {
    Ok,
    Failed,
    Released,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::RatingChangeReason")]
pub enum RatingChangeReason {
    Win,
    Loss,
    Decay,
    Placement,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::PenaltyType")]
pub enum PenaltyType {
    Leaver,
    AFK,
    Abuse,
    Cheating,
}

#[derive(DbEnum, QueryId, Debug, Clone)]
#[db_enum(existing_type_path = "crate::schema::sql_types::ReportReason")]
pub enum ReportReason {
    Smurfing,
    Cheating,
    Ghosting,
    Abuse,
}
