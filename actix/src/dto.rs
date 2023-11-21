use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePoll {
    pub title: String,
    pub description: String,
    pub expiration: String,
    pub timezone: String,
    pub lasts_for: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct Poll {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub expiration: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct PollOption {
    optionId: i32,
    pollId: i32,
    option: String,
}

pub struct Vote {
    pollId: i32,
    userId: i32,
    optionId: i32,
    ordinal: i32,
}
