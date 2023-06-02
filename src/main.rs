pub mod constants;
mod handlers;
mod middleware;
pub mod models;
pub mod utils;

extern crate twilight_gateway;
extern crate twilight_http;
extern crate twilight_model;

use std::error::Error;

use actix_web::{web, App, HttpServer};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{ConfigBuilder, EventTypeFlags, Intents, Shard, ShardId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::dotenv().ok();
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

    let _event_loop_task = tokio::spawn(async move {
        // Initialize the tracing subscriber.
        // tracing_subscriber::fmt::init();

        let token = dotenv::var("DISCORD_TOKEN").unwrap();
        let intents = Intents::GUILD_MESSAGES | Intents::GUILDS | Intents::MESSAGE_CONTENT;
        let event_types = EventTypeFlags::THREAD_CREATE
            | EventTypeFlags::THREAD_UPDATE
            | EventTypeFlags::THREAD_DELETE;

        let config = ConfigBuilder::new(token.clone(), intents)
            .event_types(event_types)
            .build();

        let mut shard = Shard::with_config(ShardId::ONE, config);

        let cache = InMemoryCache::builder()
            .resource_types(twilight_cache_inmemory::ResourceType::CHANNEL)
            .build();

        loop {
            match shard.next_event().await {
                Ok(event) => {
                    utils::handle_tag_updates(&cache, &event)
                        .await
                        .unwrap_or(());

                    cache.update(&event);
                }
                Err(source) => {
                    println!("error receiving event: {:?}", source);
                    if source.is_fatal() {
                        break;
                    }

                    continue;
                }
            };
        }
    });

    println!("Server running at http://{}/", server_addr);

    server.await?;

    Ok(())
}
