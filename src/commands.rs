use crate::config::AppConfig;
use crate::util::{react_to_message, respond_to_slash_command};
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandOptionType,
};
use serenity::prelude::*;
use std::str::FromStr;

pub enum SlashCommands {
    Lobby,
    Delete,
}

impl FromStr for SlashCommands {
    type Err = ();

    fn from_str(input: &str) -> Result<SlashCommands, Self::Err> {
        match input {
            "lobby" => Ok(SlashCommands::Lobby),
            "delete" => Ok(SlashCommands::Delete),
            _ => Err(()),
        }
    }
}

pub struct CommandRunner {}

impl CommandRunner {
    pub async fn handle_lobby_command(
        ctx: &Context,
        command: &ApplicationCommandInteraction,
        config: &AppConfig,
    ) {
        let channel = ChannelId(config.lobby_channel_id);

        respond_to_slash_command(ctx, command, "Accepted.").await;

        // Create the embed
        let result = channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| e.title("10 Man EOI").description("skrrt"))
                    .add_file("./media/jonadello.png")
            })
            .await;

        match result {
            Ok(mut message) => {
                println!("Embed Message ID: {}", message.id);
                let id = message.id.clone();
                // Update footer with message ID
                let _edit_result = message
                    .edit(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("10 Man EOI")
                                .description("skrrt")
                                .thumbnail("attachment://jonadello.png")
                                .color(0xff7700)
                                .footer(|f| f.text(id))
                        })
                    })
                    .await;

                //Add reaction options
                react_to_message(
                    ctx,
                    &message,
                    std::env::var("EMOJI_YES_ID")
                        .expect("Expected EMOJI_YES_ID in .env")
                        .parse()
                        .expect("EMOJI_YES_ID must be an integer"),
                    std::env::var("EMOJI_YES_NAME").expect("Expected EMOJI_YES_NAME in .env"),
                )
                .await;
                react_to_message(
                    ctx,
                    &message,
                    std::env::var("EMOJI_NO_ID")
                        .expect("Expected EMOJI_NO_ID in .env")
                        .parse()
                        .expect("EMOJI_NO_ID must be an integer"),
                    std::env::var("EMOJI_NO_NAME").expect("Expected EMOJI_NO_NAME in .env"),
                )
                .await;
                react_to_message(
                    ctx,
                    &message,
                    std::env::var("EMOJI_MAYBE_ID")
                        .expect("Expected EMOJI_MAYBE_ID in .env")
                        .parse()
                        .expect("EMOJI_MAYBE_ID must be an integer"),
                    std::env::var("EMOJI_MAYBE_NAME").expect("Expected EMOJI_MAYBE_NAME in .env"),
                )
                .await;
            }
            Err(error) => println!(
                "Failed to find channel to post new lobby to. Looking for channel with id {}",
                error
            ),
        }
    }

    pub async fn handle_delete_command(
        ctx: &Context,
        command: &ApplicationCommandInteraction,
        config: &AppConfig,
    ) {
        let channel = ChannelId(config.lobby_channel_id);

        let option = command.data.options.get(0).expect("Expected message ID");
        if let ApplicationCommandOptionType::String = option.kind {
            match &option.value {
                Some(message_id) => {
                    println!("str version {}", message_id);
                    let id = MessageId(message_id.as_u64().unwrap());
                    println!("thingo {}", id);
                    match channel.delete_message(&ctx.http, id).await {
                        Ok(_) => {
                            respond_to_slash_command(ctx, command, "Message deleted").await;
                        }
                        Err(_) => {
                            respond_to_slash_command(
                                ctx,
                                command,
                                format!("Failed to delete message with ID: {}", message_id),
                            )
                            .await
                        }
                    }
                }
                _ => {
                    respond_to_slash_command(ctx, command, "Failed to parse id parameter").await;
                    panic!("Failed to parse id parameter")
                }
            }
        }
    }
}
