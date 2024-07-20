use poise::serenity_prelude as serenity;

use serenity::{CreateEmbed, Color};
use poise::{CreateReply};

use crate::utils::prelude::{Context, Error};

pub(crate) async fn voice_check(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();

    if lava_client.get_player_context(guild_id).is_none() {
        let embed = CreateEmbed::default()
            .title("Ошибка")
            .description("Добавьте сначала бота в голосовой канал.")
            .color(Color::RED);

        let builder = CreateReply::default().ephemeral(true).embed(embed);
        let _ = ctx.send(builder).await;
        return Err("Бот не добавлен в голосовой канал.".into());
    }

    Ok(true)
}