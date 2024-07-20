use lavalink_rs::client::LavalinkClient;

#[derive(Debug, Clone)]
pub struct Data {
    pub lavalink: LavalinkClient,
}


pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;