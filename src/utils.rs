use std::env;

use lazy_static::__Deref;
use twilight_gateway::Event;
use twilight_http::Client as HttpClient;
use twilight_model::{id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
}, channel::Channel};

use crate::{models::{JiraIssue, IssueFields, IssueType, Project, ParsedMessageURL}, constants};

pub async fn send_update_to_user_report(
    channel_id: u64,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new HTTP client using your Discord bot token
    let http = HttpClient::new(env::var("DISCORD_TOKEN")?);

    // Send the message to the bug report channel
    let request = http.create_message(Id::<ChannelMarker>::new(channel_id)).content(message)?.await?;

    if request.status().is_success() {
        println!("Message sent successfully");
    } else {
        println!("Failed to send message");
    }

    Ok(())
}

/// The format will look like the following:
/// "[Report Origin](https://discord.com/channels/123456789/987654321/987654321)"
pub fn parse_message_url_from_issue_update(description: &String) -> ParsedMessageURL {
    let start_index = description.find("[Report Origin](").unwrap();
    let end_index = description.find(")").unwrap() - 1;

    // get the substring
    let message_url = &description[start_index..end_index];

    // split the string by "/"
    let message_url_parts: Vec<&str> = message_url.split("/").collect();

    // get the server ID
    let server_id = message_url_parts[4];

    // get the channel ID
    let channel_id = message_url_parts[5];

    // get the message ID
    let message_id = message_url_parts[6];

    // create the message URL
    ParsedMessageURL {
        server_id: Id::<GuildMarker>::new(server_id.parse::<u64>().unwrap()),
        channel_id: Id::<ChannelMarker>::new(channel_id.parse::<u64>().unwrap()),
        message_id: Id::<MessageMarker>::new(message_id.parse::<u64>().unwrap()),
    }
}

pub async fn handle_tag_updates(event: Event, channel: &Channel) -> Result<(), Box<dyn std::error::Error>> {
    // Handle the tag update event
    if let Event::ChannelUpdate(new_channel) = event {
        println!("channel event received");
        // ensure the channel is a thread within the bug report channel
        if new_channel.parent_id.unwrap() != *constants::BUG_REPORT_CHANNEL_ID {
            return Ok(());
        }

        // Check if the message contains a tag update
        // Implement your logic here to extract and process the tag update information

        // compare tags between new_channel and channel,
        // if the new_channel has the applied tag that matches the id from JIRA_SYNC_TAG_ID and the old channel does not,
        // then send the report as a new jira issue and send a message to the user report channel stating that the report is now synced to jira
        let new_channel_tags = new_channel.applied_tags.as_ref().unwrap();
        let old_channel_tags = channel.applied_tags.as_ref().unwrap();

        // get the tag id from the env var
        let tag_id = dotenv::var("JIRA_SYNC_TAG_ID").unwrap().parse::<u64>().unwrap();

        // check if the new channel has the tag
        let new_channel_has_tag = new_channel_tags.iter().any(|tag| tag.get() == tag_id);

        // check if the old channel has the tag
        let old_channel_has_tag = old_channel_tags.iter().any(|tag| tag.get() == tag_id);

        // check if the new channel has the tag and the old channel does not
        if new_channel_has_tag && !old_channel_has_tag {
            create_jira_issue(channel).await?;
        }
    }

    Ok(())
}

pub async fn create_jira_issue(channel: &Channel) -> Result<(), Box<dyn std::error::Error>> {
    // fetch the first message in the thread/post via fetching for a message within the channel using the id of the channel
    // since the starter message and post id are the same
    let http = HttpClient::new(constants::DISCORD_TOKEN.to_string());
    let message = http.message(channel.id, Id::<MessageMarker>::new(channel.id.get())).await?.model().await?;

    // use reqwest to create a new Jira issue
    let channel_name = channel.name.clone().unwrap_or_else(|| format!("Bug Report from Post ID: {}", channel.id.get()));

    let client = reqwest::Client::new();
    let response = client.post("https://computerlunch.atlassian.net/rest/api/2/issue")
        .basic_auth(env::var("JIRA_USERNAME")?, Some(env::var("JIRA_TOKEN")?))
        .json(&JiraIssue {
            fields: IssueFields {
                project: Project::default(),
                summary: channel_name,
                description: format!("[Report Origin](https://discord.com/channels/{}/{}/{})\n\n{}", message.guild_id.unwrap().get(), message.channel_id.get(), message.id.get(), message.content),
                issuetype: IssueType::default(),
                status_category: None,
            }
        })
        .send()
        .await?;

    Ok(())
}
