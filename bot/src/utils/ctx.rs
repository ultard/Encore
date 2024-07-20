use crate::utils::prelude::{Context};
use crate::utils::embeds;

use poise::{async_trait, CreateReply, ReplyHandle};
use poise::serenity_prelude as serenity;

use serenity::{
    CreateEmbed,
    GuildChannel,
};

use anyhow::Context as anyCtx;

use std::{fmt::Display};
type StdResult<T, E> = std::result::Result<T, E>;

#[extend::ext(name = PoiseContextExt)]
#[async_trait]
pub impl<'a> Context<'a> {
    fn is_prefix(&self) -> bool {
        matches!(self, poise::Context::Prefix(_))
    }

    /// Reply with an ephemeral embed.
    async fn reply_embed_ephemeral_builder(
        &self,
        build: impl FnOnce(CreateEmbed) -> CreateEmbed + Send + Sync,
    ) -> StdResult<ReplyHandle<'_>, serenity::Error> {
        self.reply_embed_ephemeral(build(embeds::base_embed())).await
    }

    /// Reply with an embed.
    async fn reply_embed_builder(
        &self,
        build: impl FnOnce(CreateEmbed) -> CreateEmbed + Send + Sync,
    ) -> StdResult<ReplyHandle<'_>, serenity::Error> {
        self.reply_embed(build(embeds::base_embed())).await
    }

    /// Reply with an embed.
    async fn reply_embed(&self, embed: CreateEmbed) -> StdResult<ReplyHandle<'_>, serenity::Error> {
        let reply = CreateReply::default().ephemeral(false).embed(embed).reply(true);
        self.send(reply).await
    }

    /// Reply with an ephemeral embed.
    async fn reply_embed_ephemeral(
        &self,
        embed: CreateEmbed,
    ) -> StdResult<ReplyHandle<'_>, serenity::Error> {
        let reply = CreateReply::default().ephemeral(true).embed(embed).reply(true);
        self.send(reply).await
    }

    async fn say_success(
        &self,
        text: impl Display + Send + Sync + 'static,
    ) -> StdResult<ReplyHandle<'_>, serenity::Error> {
        tracing::info!(
            msg.ephemeral = true,
            msg.content = %text,
            msg.responding_to_user = %self.author().tag(),
            "Sending success message to user"
        );
        self.reply_embed_ephemeral(
            embeds::make_success_embed(&text.to_string()).await,
        )
            .await
    }

    async fn say_error(
        &self,
        text: impl Display + Send + Sync + 'static,
    ) -> StdResult<ReplyHandle<'_>, serenity::Error> {
        tracing::info!(
            msg.ephemeral = true,
            msg.content = %text,
            msg.responding_to_user = %self.author().tag(),
            "Sending error message to user"
        );
        self.reply_embed_ephemeral(
            embeds::make_error_embed(&text.to_string()).await,
        )
            .await
    }

    async fn guild_channel(&self) -> anyhow::Result<GuildChannel> {
        Ok(self
            .channel_id()
            .to_channel(&self.serenity_context())
            .await
            .context("Failed to load channel")?
            .guild()
            .context("Failed to load GuildChannel")?)
    }
}