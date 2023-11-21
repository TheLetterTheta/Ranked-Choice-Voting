use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct User {
    user_id: i32,
    user_name: String,
    display_name: Option<String>,
}

#[derive(FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Poll {
    poll_id: i32,
    title: String,
    description: String,
    created_by_user_id: i32,
    created_timestamp: DateTime<Utc>,
    close_timestamp: DateTime<Utc>,
    rounds: i32,
}

#[derive(FromRow)]
pub struct CreatePollResponse {
    pub id: i32,
}

#[derive(FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct PollOption {
    poll_option_id: i32,
    poll_id: i32,
    title: String,
    description: Option<String>,
}

#[derive(FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Vote {
    vote_id: i32,
    user_id: i32,
    poll_id: i32,
    poll_option_id: i32,
    ordinal: i32,
}
