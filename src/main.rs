pub mod constants;
mod handlers;
pub mod models;
pub mod utils;
mod middleware;

extern crate twilight_gateway;
extern crate twilight_http;
extern crate twilight_model;

use std::error::Error;

use actix_web::{
    guard,
    web::{self, service},
    App, HttpServer,
};
use twilight_gateway::{ConfigBuilder, EventTypeFlags, Intents, Shard, ShardId};
use twilight_model::id::{marker::ChannelMarker, Id};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::dotenv().ok();
    println!("env vars: {:?}", dotenv::vars());
    let server_addr = dotenv::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let server = HttpServer::new(move || {
        App::new().service(
            web::scope("/discord-jira")
            .wrap(middleware::LoggingRoute {})
                // .guard(guard::Header("content-type", "application/json"))
                // .guard(guard::Post())
                .service(handlers::jira_issue_update)
                .service(handlers::testing),
        )
    })
    .bind(server_addr.clone())?
    .run();

    let event_loop_task = tokio::spawn(async move {
        // Initialize the tracing subscriber.
        // tracing_subscriber::fmt::init();

        let token = dotenv::var("DISCORD_TOKEN").unwrap();
        let intents = Intents::GUILD_MESSAGES;
        let event_types = EventTypeFlags::CHANNEL_UPDATE;
        println!("before config");
        let config = ConfigBuilder::new(token.clone(), intents)
            .event_types(event_types)
            .build();
        println!("config created");
        let mut shard = Shard::with_config(ShardId::ONE, config);
        println!("shard created");
        // tracing::info!("created shard");

        // create http client and fetch for the channel with the id inside of BUG_REPORT_CHANNEL_ID
        let http = twilight_http::Client::new(token);
        println!("http client created");
        let channel = http
            .channel(Id::<ChannelMarker>::new(*constants::BUG_REPORT_CHANNEL_ID))
            .await.unwrap()
            .model()
            .await.unwrap();
        println!("channel fetched");
        loop {
            let event = match shard.next_event().await {
                Ok(event) => utils::handle_tag_updates(event, &channel)
                    .await
                    .map_err(|source| {
                        // tracing::warn!(?source, "error handling event");

                        source
                    }),
                Err(source) => {
                    // tracing::warn!(?source, "error receiving event");
                    println!("error receiving event: {:?}", source);
                    if source.is_fatal() {
                        break;
                    }

                    continue;
                }
            };

            // tracing::debug!(?event, "event");
        }
    });

    println!("Server running at http://{}/", server_addr);

    server.await?;

    Ok(())
}
