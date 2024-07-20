use std::time::Duration;
use std::ops::Deref;

use futures::future;
use futures::stream::StreamExt;
use lavalink_rs::prelude::*;

use crate::abort_with;
use crate::checker::*;
use crate::utils::{
    prelude::{Context, Error},
    ctx::PoiseContextExt
};

use poise::{CreateReply};
use poise::serenity_prelude as serenity;
use serenity::{
    model::id::ChannelId,
    Http, CreateEmbed, Color, Mentionable
};

async fn _join(
    ctx: &Context<'_>,
    guild_id: serenity::GuildId,
    channel_id: Option<ChannelId>,
) -> Result<bool, Error> {
    let lava_client = ctx.data().lavalink.clone();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if lava_client.get_player_context(guild_id).is_none() {
        let connect_to = match channel_id {
            Some(x) => x,
            None => {
                let guild = ctx.guild().unwrap().deref().clone();
                let user_channel_id = guild
                    .voice_states
                    .get(&ctx.author().id)
                    .and_then(|voice_state| voice_state.channel_id);

                match user_channel_id {
                    Some(channel) => channel,
                    None => {
                        #[allow(unreachable_code)]
                        return abort_with!("Вы не в голосовом канале");
                    }
                }
            }
        };

        let handler = manager.join_gateway(guild_id, connect_to).await;

        return match handler {
            Ok((connection_info, _)) => {
                lava_client
                    .create_player_context_with_data::<(ChannelId, std::sync::Arc<Http>)>(
                        guild_id,
                        connection_info,
                        std::sync::Arc::new((
                            ctx.channel_id(),
                            ctx.serenity_context().http.clone(),
                        )),
                    )
                    .await?;

                let embed = CreateEmbed::default()
                    .title("Подключен!")
                    .description(format!("Бот присоединился к каналу {}.", connect_to.mention()))
                    .color(Color::DARK_GREEN);

                let builder = CreateReply::default().ephemeral(true).embed(embed);
                let _ = ctx.send(builder).await;

                Ok(true)
            }
            Err(why) => {
                ctx.say_error(format!("Не удалось присоединится к каналу: {}", why))
                    .await?;
                Err(why.into())
            }
        }
    }

    Ok(false)
}

/// Воспроизведение трека в вашем голосовом канале.
#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Поисковый запрос или URL-адрес"]
    #[rest]
    term: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();

    _join(&ctx, guild_id, None).await?;

    let Some(player) = lava_client.get_player_context(guild_id) else {
        ctx.say_error("Добавьте сначала бота в голосовой канал.").await?;
        return Ok(());
    };

    let query = if let Some(term) = term {
        if term.starts_with("http") {
            term
        } else {
            SearchEngines::YouTube.to_query(&term)?
        }
    } else {
        if let Ok(player_data) = player.get_player().await {
            let queue = player.get_queue();

            if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
                player.skip()?;
            } else {
                ctx.say("The queue is empty.").await?;
            }
        }

        return Ok(());
    };

    let loaded_tracks = lava_client.load_tracks(guild_id, &query).await?;

    let mut playlist_info = None;
    let mut tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }

        _ => {
            ctx.say(format!("{:?}", loaded_tracks)).await?;
            return Ok(());
        }
    };

    for i in &mut tracks {
        i.track.user_data = Some(serde_json::json!({"requester_id": ctx.author().id.get()}));
    }

    let queue = player.get_queue();
    queue.append(tracks.clone().into())?;

    let mut embed = CreateEmbed::new();

    if let Some(info) = playlist_info {
        embed = embed
            .title("Плейлист")
            .description(format!("Добавлен плейлист в очередь: {}", info.name));
    } else {
        let track = &tracks[0].track;

        if let Some(uri) = &track.info.uri {
            embed = embed
                .title("Песня")
                .description(format!(
                    "Добавлена в очередь: [{} - {}](<{}>)",
                    track.info.author, track.info.title, uri
                ));
        } else {
            embed = embed
                .title("Песня")
                .description(format!(
                    "Добавлена в очередь: {} - {}",
                    track.info.author, track.info.title
                ));
        }
    }

    let builder = CreateReply::default().ephemeral(true).embed(embed);
    let _ = ctx.send(builder).await;

    Ok(())
}

/// Подключить бота к голосовому каналу.
#[poise::command(slash_command, prefix_command)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Айди канала для присоединения."]
    #[channel_types("Voice")]
    channel_id: Option<ChannelId>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let status = _join(&ctx, guild_id, channel_id).await?;

    if status == false {
        abort_with!("Бот уже подключён к каналу");
    };

    Ok(())
}
/// Выйти из текущего голосового канала.
#[poise::command(slash_command, prefix_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if lava_client.get_player_context(guild_id).is_some() {
        lava_client.delete_player(guild_id).await?;
    };

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
    }

    ctx.say_success("Бот отключён от канала.").await?;

    Ok(())
}

/// Меняет громкость проигрываемой музыки.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn volume(
    ctx: Context<'_>,
    #[description = "Новая громкость"]
    #[min = 0]
    #[max = 200]
    level: u16
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    player.set_volume(level).await?;
    let embed = CreateEmbed::default()
        .title("Громкость")
        .description(format!("Изменена громкость на {}%.", level))
        .color(Color::DARK_GREEN);

    let builder = CreateReply::default().ephemeral(true).embed(embed);
    let _ = ctx.send(builder).await;

    Ok(())
}

/// Показывает текущую очередь треков.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    let player_data = player.get_player().await?;
    let mut now_embed = CreateEmbed::default()
        .title("Сейчас играет".to_string());

    if let Some(track) = player_data.track {
        let time_s = player_data.state.position / 1000 % 60;
        let time_m = player_data.state.position / 1000 / 60;
        let time = format!("{:02}:{:02}", time_m, time_s);

        now_embed = now_embed
            .field("Добавил", format!("<@{}>", track.user_data.clone().unwrap()["requester_id"]), true)
            .field("Автор", &track.info.author, true)
            .field("Время", time, true);

        if let Some(artwork) = &track.info.artwork_url {
            now_embed = now_embed
                .thumbnail(artwork);
        }

        if let Some(uri) = &track.info.uri {
            now_embed = now_embed.description(format!(
                "[{}](<{}>)",
                track.info.title,
                uri
            ));
        } else {
            now_embed = now_embed.description(format!("`{}`", track.info.title));
        }
    } else {
        now_embed = now_embed.description("Ничего")
    };

    let builder = CreateReply::default()
        .ephemeral(true)
        .embed(now_embed);
    let _ = ctx.send(builder).await;

    Ok(())
}

/// Показывает текущую очередь треков.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    let queue = player.get_queue();

    let max = queue.get_count().await?.min(9);
    let queue_message = queue
        .enumerate()
        .take_while(|(idx, _)| future::ready(*idx < max))
        .map(|(idx, x)| {
            if let Some(uri) = &x.track.info.uri {
                format!(
                    "**{}.** [{} - {}](<{}>)",
                    idx + 1,
                    x.track.info.author,
                    x.track.info.title,
                    uri
                )
            } else {
                format!(
                    "**{}.** {} - {}",
                    idx + 1,
                    x.track.info.author,
                    x.track.info.title
                )
            }
        })
        .collect::<Vec<_>>()
        .await
        .join("\n");

    let queue_embed = CreateEmbed::default()
        .title("Очередь треков")
        .description(queue_message);

    let builder = CreateReply::default()
        .ephemeral(true)
        .embed(queue_embed);
    let _ = ctx.send(builder).await;

    Ok(())
}

/// Прпопускает текущий трек.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    let now_playing = player.get_player().await?.track;

    if let Some(np) = now_playing {
        player.skip()?;
        ctx.say_success(format!("Пропущен {}", np.info.title)).await?;
    } else {
        ctx.say_error("Нечего пропускать").await?;
    }

    Ok(())
}

/// Pause the current song.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    player.set_pause(true).await?;
    ctx.say_success("Приостановлено").await?;
    Ok(())
}

/// Resume playing the current song.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };


    player.set_pause(false).await?;
    ctx.say_success("Возобновлено воспроизведение").await?;
    Ok(())
}

/// Stops the playback of the current song.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    let now_playing = player.get_player().await?.track;

    if let Some(np) = now_playing {
        player.stop_now().await?;
        ctx.say(format!("Остановлен {}", np.info.title)).await?;
    } else {
        ctx.say_error("Нечего останавливать").await?;
    }

    Ok(())
}

/// Jump to a specific time in the song, in seconds.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn seek(
    ctx: Context<'_>,
    #[description = "Time to jump to (in seconds)"] time: u64,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    let now_playing = player.get_player().await?.track;

    if now_playing.is_some() {
        player.set_position(Duration::from_secs(time)).await?;
        ctx.say_success(format!("Переход к {}c", time)).await?;
    } else {
        ctx.say_error("Ничего не играет").await?;
    }

    Ok(())
}

/// Remove a specific song from the queue.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Queue item index to remove"] index: usize,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    player.get_queue().remove(index)?;
    ctx.say_success("Трек удалён").await?;

    Ok(())
}

/// Clear the current queue.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    player.get_queue().clear()?;
    ctx.say_success("Очередь очищена").await?;

    Ok(())
}

/// Swap between 2 songs in the queue.
#[poise::command(slash_command, prefix_command, check = "voice_check")]
pub async fn swap(
    ctx: Context<'_>,
    #[description = "Queue item index to swap"] index1: usize,
    #[description = "The other queue item index to swap"] index2: usize,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let Some(player) = lava_client.get_player_context(guild_id) else { todo!() };

    let queue = player.get_queue();
    let queue_len = queue.get_count().await?;

    if index1 > queue_len || index2 > queue_len {
        ctx.say_error(format!("Максимально разрешённаый номер: {}", queue_len))
            .await?;
        return Ok(());
    } else if index1 == index2 {
        ctx.say_error("Нельзя поменять между одинаковыми номерами").await?;
        return Ok(());
    }

    let track1 = queue.get_track(index1 - 1).await?.unwrap();
    let track2 = queue.get_track(index1 - 2).await?.unwrap();

    queue.swap(index1 - 1, track2)?;
    queue.swap(index2 - 1, track1)?;

    ctx.say_success("Места изменены").await?;

    Ok(())
}

