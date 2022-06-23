extern crate dotenv;
extern crate tokio;

#[macro_use]
extern crate lazy_static;

pub mod commands;
pub mod config;
pub mod reactions;
pub mod sheets_manager;
pub mod util;

use crate::commands::{CommandRunner, SlashCommands};
use crate::config::{load_config, AppConfig};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::interactions::{
    application_command::{ApplicationCommand, ApplicationCommandOptionType},
    Interaction,
};
use serenity::prelude::*;
use sheets_manager::SheetsManager;
use util::{check_for_senders_role, respond_to_slash_command};

use std::str::FromStr;

lazy_static! {
    static ref CONFIG: AppConfig = load_config();
}

static DEFAULT_LIST_STRING: &str = "...";

struct Handler {
    sheets_manager: SheetsManager,
}

impl Handler {
    async fn new(sheets_manager: SheetsManager) -> Handler {
        sheets_manager.initialise("My new sheet".to_string()).await;
        Handler { sheets_manager }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // Handle Slash Commands
            if !(check_for_senders_role(&ctx, &command, CONFIG.guild_id, CONFIG.organiser_role_id)
                .await)
            {
                respond_to_slash_command(
                    &ctx,
                    &command,
                    "You're not powerful enough...weakling...........",
                )
                .await;
                return;
            }
            let raw_command_name = command.data.name.as_str();

            match SlashCommands::from_str(raw_command_name) {
                Ok(SlashCommands::Lobby) => {
                    CommandRunner::handle_lobby_command(&ctx, &command, &CONFIG).await
                }
                Ok(SlashCommands::Delete) => {
                    CommandRunner::handle_delete_command(&ctx, &command, &CONFIG).await
                }
                Err(_) => println!("Unknown command: {}", raw_command_name),
            };
        } else if let Interaction::MessageComponent(reaction) = interaction {
            // Handle component reactions (embed button presses etc)
            if reaction.channel_id == CONFIG.lobby_channel_id {
                CommandRunner::handle_signup_response(&ctx, reaction, &CONFIG).await;
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _global_command =
            ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
                commands
                    .create_application_command(|f| {
                        f.name("lobby")
                            .description("Create a new scrim lobby signup sheet")
                            .create_option(|option| {
                                option
                                    .name("when")
                                    .description("The time for gaming")
                                    .kind(ApplicationCommandOptionType::String)
                                    .required(true)
                            })
                    })
                    .create_application_command(|f| {
                        f.name("delete")
                            .description("Delete a signup sheet by ID")
                            .create_option(|option| {
                                option
                                    .name("id")
                                    .description("The ID of the message to delete")
                                    .kind(ApplicationCommandOptionType::String)
                                    .required(true)
                            })
                    })
            })
            .await;
    }
}

#[tokio::main]
async fn main() {
    let sheets_client: SheetsManager = sheets_manager::login(
        CONFIG.google_sheets_client_id.clone(),
        CONFIG.google_sheets_client_secret.clone(),
    )
    .await;

    // Build our client.
    let mut client = Client::builder(
        &CONFIG.discord_token,
        GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_MESSAGE_REACTIONS,
    )
    .event_handler(Handler::new(sheets_client).await)
    .await
    .expect("ErroONFIGcreating client");

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
