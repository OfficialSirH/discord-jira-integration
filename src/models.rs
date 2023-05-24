use serde::{Deserialize, Serialize};
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};

use crate::constants::StatusCategoryId;

pub struct ParsedMessageURL {
    pub server_id: Id<GuildMarker>,
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JiraIssue {
    pub fields: IssueFields,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IssueFields {
    pub project: Project,
    pub summary: String,
    pub description: String,
    pub issuetype: IssueType,
    #[serde(rename = "statusCategory")]
    pub status_category: Option<StatusCategory>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub key: String,
    pub id: String,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            key: String::from("Cells Dev"),
            id: String::from("10000"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IssueType {
    pub name: Option<String>,
    pub id: String,
}

impl Default for IssueType {
    fn default() -> Self {
        IssueType {
            name: None,
            id: String::from("10000"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusCategory {
    pub id: StatusCategoryId,
    pub key: String,
    #[serde(rename = "colorName")]
    pub color_name: String,
    pub name: String,
}
