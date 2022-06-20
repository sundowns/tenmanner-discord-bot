use std::vec;

use serenity::model::interactions::message_component::MessageComponentInteraction;
use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
};
use serenity::model::misc::EmojiIdentifier;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::reactions::GamerResponseOption;
use crate::DEFAULT_LIST_STRING;

pub async fn react_to_message(ctx: &Context, message: &Message, emoji_id: u64, emoji_name: String) {
    let _result = message
        .react(
            &ctx.http,
            EmojiIdentifier {
                animated: false,
                id: EmojiId(emoji_id),
                name: emoji_name,
            },
        )
        .await;
}

pub async fn respond_to_slash_command<D: ToString>(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    message_content: D,
) {
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content(message_content).ephemeral(true)
                })
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}

pub async fn respond_to_signup_interaction<D: ToString>(
    ctx: &Context,
    reaction: &MessageComponentInteraction,
    message_content: D,
) {
    if let Err(why) = reaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content(message_content).ephemeral(true)
                })
        })
        .await
    {
        println!("Cannot respond to signup response: {}", why);
    }
}

pub async fn check_for_senders_role(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    guild_id: u64,
    role_id: u64,
) -> bool {
    return match command
        .user
        .has_role(&ctx.http, GuildId(guild_id), RoleId(role_id))
        .await
    {
        Ok(has_role) => has_role,
        Err(_) => {
            println!(
                "Failed to lookup user's role with guild ID [{}] and role ID [{}]",
                guild_id, role_id
            );
            false
        }
    };
}

pub async fn strip_mention_from_response_lists(
    data: Vec<EmbedField>,
    user_id: UserId,
) -> Vec<(String, String, bool)> {
    let mut return_data: Vec<(String, String, bool)> = vec![];
    // Loop over the data, for each collection
    for field in data {
        if field.value.contains(&user_id.to_string()) {
            println!("we should strip mention from {}", field.name);
            // Includes an existing mention, remove it and add the data
            let removed: String = field
                .value
                .split(" ")
                .filter(|p| p.contains(&user_id.to_string()))
                .collect();
            println!("removed {}", removed);
            // If it is now empty, change to DEFAULT_LIST_STRING
            return_data.push((
                field.name,
                if removed.is_empty() {
                    DEFAULT_LIST_STRING.to_string()
                } else {
                    removed
                },
                field.inline,
            ))
        } else {
            // Doesn't contain mention, just include it
            return_data.push(to_tuple(field));
        }
    }
    return_data
}

pub async fn add_mention_to_response_list(
    data: Vec<(String, String, bool)>,
    add_to: GamerResponseOption,
    user_id: UserId,
) -> Vec<(String, String, bool)> {
    let mut return_data: Vec<(String, String, bool)> = vec![];
    // Find tuple in collection that matches `add_to` option
    for field in data {
        if field.0 == add_to.heading() {
            // Add users mention
            let mention = Mention::User(user_id).to_string();
            let value = if field.1 == DEFAULT_LIST_STRING {
                // If it was empty before, just set it to the mention
                mention
            } else {
                // otherwise append it on thened
                format!("{} {}", field.1, mention)
            };
            return_data.push((field.0, value, field.2));
        } else {
            return_data.push(field);
        }
    }
    return_data
}

pub fn to_tuple(from: EmbedField) -> (String, String, bool) {
    (from.name, from.value, from.inline)
}
