use tracing::{info};
use lavalink_rs::{hook, model::events, prelude::*};
use poise::serenity_prelude::{model::id::ChannelId, Http, CreateEmbed, CreateMessage};

#[hook]
pub async fn raw_event(_: LavalinkClient, session_id: String, event: &serde_json::Value) {
    if event["op"].as_str() == Some("event") || event["op"].as_str() == Some("playerUpdate") {
        info!("{:?} -> {:?}", session_id, event);
    }
}

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    info!("{:?} -> {:?}", session_id, event);
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &events::TrackStart) {
    let player_context = client.get_player_context(event.guild_id).unwrap();
    let data = player_context
        .data::<(ChannelId, std::sync::Arc<Http>)>()
        .unwrap();
    let (channel_id, http) = (&data.0, &data.1);

    let track = &event.track;

    let time_s = track.info.length / 1000 % 60;
    let time_m = track.info.length / 1000 / 60;
    let time = format!("{:02}:{:02}", time_m, time_s);

    let mut embed = CreateEmbed::default()
        .title("Сейчас играет".to_string())
        .field("Добавил", format!("<@{}>", track.user_data.clone().unwrap()["requester_id"]), true)
        .field("Автор", &track.info.author, true)
        .field("Длительность", time, true);

    if let Some(artwork) = &track.info.artwork_url {
        embed = embed
            .thumbnail(artwork);
    }

    if let Some(uri) = &track.info.uri {
        embed = embed.description(format!(
            "[{}](<{}>)",
            track.info.title,
            uri
        ));
    } else {
        embed = embed.description(format!("`{}`", track.info.title));
    }

    let builder = CreateMessage::new().embed(embed);
    let _ = channel_id.send_message(http, builder).await;
}
