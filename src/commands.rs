use crate::config::AppConfig;
use crate::util::respond_to_slash_command;
use serenity::builder::{CreateActionRow, CreateButton};
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandOptionType,
};
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::prelude::*;
use std::fmt;
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

enum GamerResponseOption {
    Yes,
    No,
    Maybe,
    Late,
}

impl fmt::Display for GamerResponseOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yes => write!(f, "Yes"),
            Self::No => write!(f, "No"),
            Self::Maybe => write!(f, "Maybe"),
            Self::Late => write!(f, "Late"),
        }
    }
}

impl GamerResponseOption {
    fn emoji(&self) -> char {
        match self {
            Self::Yes => '✅',
            Self::No => '❌',
            Self::Maybe => '❔',
            Self::Late => '⌛',
        }
    }

    fn button(&self) -> CreateButton {
        let mut b = CreateButton::default();
        b.custom_id(self.to_string().to_ascii_lowercase());
        b.emoji(self.emoji());
        b.label(self);
        b.style(ButtonStyle::Primary);
        b
    }

    fn action_row() -> CreateActionRow {
        let mut ar = CreateActionRow::default();
        // We can add up to 5 buttons per action row
        ar.add_button(GamerResponseOption::Yes.button());
        ar.add_button(GamerResponseOption::No.button());
        ar.add_button(GamerResponseOption::Maybe.button());
        ar.add_button(GamerResponseOption::Late.button());
        ar
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
                        .components(|f| f.add_action_row(GamerResponseOption::action_row()))
                    })
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
}
