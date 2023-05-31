use actix_web::{get, post, web::Bytes, HttpRequest, HttpResponse};

#[post("")]
pub async fn jira_issue_update(
    _request: HttpRequest,
    bytes: Bytes,
) -> Result<HttpResponse, actix_web::Error> {
    // turn actix bytes to string then print
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    println!("body: {:?}", body);

    // using the Jira API, we're listening for webhook requests about issue updates
    // parse the description of the issue update to get the message URL
    // send a message to the channel where the bug report originated with the issue update
    // return a 200 OK response

    // let parsed_description = crate::utils::parse_message_url_from_issue_update(&data.issue.fields.description);

    // let message = format!("Your bug report has been updated. The new status is {}.", data.issue.fields.status.name);

    // crate::utils::send_update_to_user_report(parsed_description.channel_id.get(), &message).await.unwrap();

    Ok(HttpResponse::Ok().body("OK"))
}

#[get("")]
pub async fn testing() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("OK"))
}
