use crate::config::AppConfig;
use serenity::model::id::{ChannelId, EmojiId};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::misc::EmojiIdentifier;
use serenity::prelude::*;

use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
    static ref EMOJIS: HashMap<&'static str, u64> = {
        let mut m = HashMap::new();
        m.insert("MOE", 936941675811582032);
        m
    };
}

pub struct CommandRunner {}

impl CommandRunner {
    pub async fn handle_lobby_command(
        ctx: &Context,
        _command: &ApplicationCommandInteraction,
        config: &AppConfig,
    ) -> String {
        let channel = ChannelId(config.lobby_channel_id);

        let result = channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("10 Man EOI")
                        .description("skrrt")
                        .thumbnail("attachment://jonadello.png")
                })
                .add_file("./media/jonadello.png")
            })
            .await;

        match result {
            Ok(message) => {
                println!("ID: {}", message.id);
                let reaction_result = message
                    .react(
                        &ctx.http,
                        EmojiIdentifier {
                            animated: false,
                            id: EmojiId(*EMOJIS.get("MOE").unwrap()),
                            name: "MOE".to_string(),
                        },
                    )
                    .await;
                match reaction_result {
                    Ok(_success) => {
                        println!("reaction success");
                    }
                    Err(error) => {
                        println!("Error when reacting: {}", error);
                    }
                }
            }
            Err(error) => println!(
                "Failed to find channel to post new lobby to. Looking for channel with id {}",
                error
            ),
        }

        return "Created!".to_string();
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
