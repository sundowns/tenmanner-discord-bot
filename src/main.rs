extern crate dotenv;
extern crate tokio;

#[macro_use]
extern crate lazy_static;

pub mod commands;
pub mod config;

use crate::commands::{CommandRunner, SlashCommands};
use crate::config::{load_config, AppConfig};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::{
    application_command::ApplicationCommand, Interaction, InteractionResponseType,
};
use serenity::prelude::*;

use std::str::FromStr;

lazy_static! {
    static ref CONFIG: AppConfig = load_config();
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let raw_command_name = command.data.name.as_str();
            let immediate_response_content: String = match SlashCommands::from_str(raw_command_name)
            {
                Ok(SlashCommands::Lobby) => {
                    CommandRunner::handle_lobby_command(&ctx, &command, &CONFIG).await
                }
                Err(_) => format!("Unknown command: {}", raw_command_name).to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(immediate_response_content).ephemeral(true)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(CONFIG.guild_id);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("lobby")
                    .description("Create a new scrim lobby signup sheet")
            })
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );

        let global_command =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("lobby")
                    .description("Create a new scrim lobby signup sheet")
            })
            .await;

        println!(
            "I created the following global slash command: {:#?}",
            global_command
        );
    }
}

#[tokio::main]
async fn main() {
    // Build our client.
    let mut client = Client::builder(&CONFIG.discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
