use serde::{Deserialize, Serialize};
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};

use crate::constants::StatusCategoryId;

pub trait AsStr {
    fn as_str(&self) -> &'static str;
}

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
    // pub description: JiraDescription,
    pub issuetype: IssueType,
    #[serde(rename = "statusCategory", skip_serializing_if = "Option::is_none")]
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
            id: String::from("10004"),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct JiraDescription {
    pub r#type: String,
    pub version: u32,
    pub content: Vec<JiraContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JiraContent {
    pub content: Vec<JiraText>,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum JiraContentType {
    Paragraph,
    Heading,
    BulletList,
}

impl AsStr for JiraContentType {
    fn as_str(&self) -> &'static str {
        match self {
            JiraContentType::Paragraph => "paragraph",
            JiraContentType::Heading => "heading",
            JiraContentType::BulletList => "bulletList",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JiraText {
    pub r#type: String,
    pub text: String,
    pub marks: Option<Vec<JiraMark>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum JiraTextType {
    Text,
}

impl AsStr for JiraTextType {
    fn as_str(&self) -> &'static str {
        match self {
            JiraTextType::Text => "text",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JiraMark {
    pub r#type: String,
    pub attrs: Option<JiraMarkAttrs>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JiraMarkAttrs {
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum JiraMarkType {
    Strong,
    Emphasis,
    Link,
    // Add other mark types as needed
}

impl AsStr for JiraMarkType {
    fn as_str(&self) -> &'static str {
        match self {
            JiraMarkType::Strong => "strong",
            JiraMarkType::Emphasis => "em",
            JiraMarkType::Link => "link",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateJiraIssueResponse {
    pub key: String,
    pub id: String,
    #[serde(rename = "self")]
    pub url: String,
}
