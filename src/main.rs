extern crate dotenv;
extern crate tokio;

#[macro_use]
extern crate lazy_static;

pub mod commands;
pub mod config;

use crate::commands::{CommandRunner, SlashCommands};
use crate::config::AppConfig;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::{
    application_command::ApplicationCommand, Interaction, InteractionResponseType,
};
use serenity::prelude::*;
use std::env;
use std::str::FromStr;

fn load_config() -> AppConfig {
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN").unwrap();
    let guild_id = env::var("GUILD_ID")
        .expect("Expected GUILD_ID in .env")
        .parse()
        .expect("GUILD_ID must be an integer");
    let lobby_channel_id = env::var("LOBBY_CHANNEL_ID")
        .expect("Expected LOBBY_CHANNEL_ID in .env")
        .parse()
        .expect("LOBBY_CHANNEL_ID must be an integer");
    AppConfig {
        discord_token,
        guild_id,
        lobby_channel_id,
    }
}

lazy_static! {
    static ref CONFIG: AppConfig = load_config();
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let immediate_response_content: String =
                match SlashCommands::from_str(command.data.name.as_str()).unwrap() {
                    SlashCommands::Lobby => {
                        CommandRunner::handle_lobby_command(&ctx, &command, &CONFIG).await
                    }
                    _ => "Unknown command".to_string(),
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
