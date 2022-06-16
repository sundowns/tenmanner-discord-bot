use crate::config::AppConfig;
use crate::util::{react_to_message, respond_to_slash_command};
use serenity::model::id::ChannelId;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use std::str::FromStr;

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
}
