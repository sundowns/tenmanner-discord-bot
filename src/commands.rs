use crate::config::AppConfig;
use serenity::model::id::ChannelId;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

use std::str::FromStr;
pub struct CommandRunner {}

impl CommandRunner {
    pub async fn handle_lobby_command(
        ctx: &Context,
        _command: &ApplicationCommandInteraction,
        config: &AppConfig,
    ) -> String {
        if let Err(result) = ChannelId(config.lobby_channel_id)
            .say(&ctx.http, "blah")
            .await
        {
            println!(
                "Failed to find channel to post new lobby to. Looking for channel with id {}",
                result
            );
        };
        return "Ok".to_string();
    }
}

pub enum SlashCommands {
    Lobby,
}

impl FromStr for SlashCommands {
    type Err = ();

    fn from_str(input: &str) -> Result<SlashCommands, Self::Err> {
        match input {
            "lobby" => Ok(SlashCommands::Lobby),
            _ => Err(()),
        }
    }
}
