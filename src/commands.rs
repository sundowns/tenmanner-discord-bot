use crate::config::AppConfig;
use crate::reactions::{handle_lobby_reaction, summarise_reactions, GamerResponseOption};
use crate::util::{respond_to_signup_interaction, respond_to_slash_command};
use crate::DEFAULT_LIST_STRING;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandOptionType,
};
use serenity::model::interactions::message_component::MessageComponentInteraction;

use serenity::prelude::*;
use std::str::FromStr;
use std::vec;

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

        let option = command.data.options.get(0).expect("Expected time option.");
        let time_string = match &option.value {
            Some(value) => value.as_str().unwrap(),
            _ => "",
        };
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
                let id = message.id.clone();
                // Update footer with message IDawait_component_interactions
                let edit_result = message
                    .edit(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title("10 Man EOI")
                                .description(format!("🕒 **{}**", time_string))
                                .thumbnail("attachment://jonadello.png")
                                .color(0xff7700)
                                .footer(|f| f.text(id))
                                .fields(vec![
                                    (
                                        GamerResponseOption::Yes.heading(),
                                        DEFAULT_LIST_STRING,
                                        false,
                                    ),
                                    (
                                        GamerResponseOption::No.heading(),
                                        DEFAULT_LIST_STRING,
                                        false,
                                    ),
                                    (
                                        GamerResponseOption::Maybe.heading(),
                                        DEFAULT_LIST_STRING,
                                        false,
                                    ),
                                    (
                                        GamerResponseOption::Late.heading(),
                                        DEFAULT_LIST_STRING,
                                        false,
                                    ),
                                ])
                        })
                        .components(|f| f.add_action_row(GamerResponseOption::action_row()))
                    })
                    .await;
                match edit_result {
                    Ok(_res) => {
                        println!("New lobby post created");
                    }
                    Err(err) => println!("Failed to edit lobby post upon creation: {}", err),
                }
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
                    let string_val = message_id.as_str().unwrap();

                    if let Ok(integer_id) = string_val.parse::<u64>() {
                        let id = MessageId(integer_id);
                        match channel.delete_message(&ctx.http, id).await {
                            Ok(_) => {
                                respond_to_slash_command(ctx, command, "Signup post deleted.")
                                    .await;
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
                    } else {
                        respond_to_slash_command(
                            ctx,
                            command,
                            "Invalid message ID - copy the value from a signup post's footer",
                        )
                        .await;
                    };
                }
                _ => {
                    respond_to_slash_command(ctx, command, "Failed to parse id parameter").await;
                    panic!("Failed to parse id parameter")
                }
            }
        }
    }

    pub async fn handle_signup_response(
        ctx: &Context,
        reaction: MessageComponentInteraction,
        _config: &AppConfig,
    ) {
        if let Ok(response) = GamerResponseOption::from_str(&reaction.data.custom_id) {
            respond_to_signup_interaction(
                ctx,
                &reaction,
                format!("{} **{}** response received.", response.emoji(), response),
            )
            .await;

            handle_lobby_reaction(ctx, reaction, response).await;
            summarise_reactions(ctx, reaction.message).await;
        } else {
            respond_to_signup_interaction(ctx, &reaction, "Failed :c").await;
        }
    }
}
