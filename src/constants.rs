use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

// lazy static for required env vars
lazy_static! {
    pub static ref DISCORD_TOKEN: String = dotenv::var("DISCORD_TOKEN").unwrap();
    pub static ref SERVER_ADDR: String =
        dotenv::var("SERVER_ADDR").unwrap_or_else(|_| "localhost:8008".to_string());
    pub static ref GUILD_ID: u64 = dotenv::var("GUILD_ID").unwrap().parse::<u64>().unwrap();
    pub static ref BUG_REPORT_CHANNEL_ID: u64 = dotenv::var("BUG_REPORT_CHANNEL_ID")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    pub static ref BETA_BUG_REPORT_CHANNEL_ID: u64 = dotenv::var("BETA_BUG_REPORT_CHANNEL_ID")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    pub static ref JIRA_SYNC_TAG: u64 = dotenv::var("JIRA_SYNC_TAG")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    pub static ref BETA_JIRA_SYNC_TAG: u64 = dotenv::var("BETA_JIRA_SYNC_TAG")
        .unwrap()
        .parse::<u64>()
        .unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StatusCategoryId {
    Backlog = 10000,
    Todo = 10005,
    InProgress = 3,
    InQA = 10006,
    Done = 10002,
}

pub enum Resolutions {
    Done = 10000,
    WonTDo = 10001,
    Duplicate = 10002,
    CannotReproduce = 10003,
}
