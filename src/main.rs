extern crate dotenv;
extern crate tokio;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::{
    application_command::ApplicationCommand, Interaction, InteractionResponseType,
};
use serenity::prelude::*;
use std::env;

#[derive(Debug)]
struct AppConfig {
    discord_token: String,
}

fn load_config() -> AppConfig {
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN").unwrap();
    AppConfig { discord_token }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("ping").description("A ping command")
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
                    .name("wonderful_command")
                    .description("An amazing command")
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
    let config: AppConfig = load_config();
    println!("{:?}", config.discord_token);
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
