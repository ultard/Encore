extern crate tracing;

pub mod checker;
pub mod errors_handler;
pub mod utils;
pub mod events;
pub mod commands;

use crate::utils::{prelude::{Error, Data}};
use lavalink_rs::prelude::*;
use lavalink_rs::model;

use poise::serenity_prelude as serenity;
use songbird::SerenityInit;
use tracing::warn;

#[tokio::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info,lavalink_rs=trace");
    tracing_subscriber::fmt::init();
    if dotenvy::dotenv().is_err() {
        warn!(".env not found, app may crash")
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all_commands(),
            on_error: |err| Box::pin(errors_handler::on_error(err)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(",".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let node_local = NodeBuilder {
                    hostname: format!("{}:{}", std::env::var("SERVER_ADDRESS").expect("host not found"), std::env::var("SERVER_PORT").expect("port not found")),
                    is_ssl: false,
                    events: model::events::Events::default(),
                    password: std::env::var("LAVALINK_SERVER_PASSWORD").expect("password not found"),
                    user_id: ctx.cache.current_user().id.into(),
                    session_id: None,
                };

                let client = LavalinkClient::new(
                    events::lava_events(),
                    vec![node_local],
                    NodeDistributionStrategy::round_robin(),
                ).await;

                Ok(Data { lavalink: client })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(
        std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"),
        serenity::GatewayIntents::all(),
    )
        .activity(serenity::gateway::ActivityData::listening("/play"))
        .status(serenity::OnlineStatus::Idle)

        .register_songbird()
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}

