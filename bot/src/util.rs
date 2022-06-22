use std::vec;

use serenity::model::interactions::message_component::MessageComponentInteraction;
use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
};
use serenity::model::misc::EmojiIdentifier;
use serenity::model::prelude::*;

use serenity::builder::CreateEmbed;
use serenity::prelude::*;

use crate::reactions::{GamerResponseOption, LobbySignupSummary, LobbyStatus, ReactionsError};
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
            // Includes an existing mention, remove it and add the data
            // TODO: figure out a better way to do this
            let removed: String = field
                .value
                .split(" ")
                .filter(|p| !p.contains(&user_id.to_string()))
                .collect();
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
    for (name, value, inline) in data {
        if name.contains(&add_to.heading()) {
            // Add users mention
            let mention = Mention::User(user_id).to_string();
            let new_value = if value == DEFAULT_LIST_STRING {
                // If it was empty before, just set it to the mention
                mention
            } else {
                // otherwise append it on thened
                format!("{} {}", value, mention)
            };
            return_data.push((name, new_value, inline));
        } else {
            return_data.push((name, value, inline));
        }
    }
    return_data
}

pub fn to_tuple(from: EmbedField) -> (String, String, bool) {
    (from.name, from.value, from.inline)
}

pub async fn update_message_embed(
    ctx: &Context,
    mut message: Message,
    response_data: Vec<(String, String, bool)>,
    summary: LobbySignupSummary,
) -> Result<(), SerenityError> {
    let status = LobbyStatus::from(summary);
    let mut existing_embed = message.embeds[0].clone();
    existing_embed.fields = vec![];
    let embed_colour = status.colour();

    let data_with_count_in_headings = add_count_to_response_headings(response_data, summary);
    // replace the new_data with the updated one
    return message
        .edit(&ctx.http, |f| {
            f.embed(|e| {
                *e = CreateEmbed::from(existing_embed);
                e.colour(embed_colour)
                    .fields(data_with_count_in_headings)
                    .thumbnail("attachment://jonadello.png")
            })
        })
        .await;
}

fn add_count_to_response_headings(
    response_data: Vec<(String, String, bool)>,
    summary: LobbySignupSummary,
) -> Vec<(String, String, bool)> {
    let mut new_data: Vec<(String, String, bool)> = vec![];
    for (name, value, inline) in response_data {
        if let Ok(response_type) = get_response_type_from_heading(name.clone()) {
            new_data.push((
                format!(
                    "{}   [{}]",
                    response_type.heading(),
                    summary.value_for_response_type(response_type)
                ),
                value,
                inline,
            ));
        } else {
            new_data.push((name, value, inline));
        }
    }
    new_data
}

// This is a hack and I'm sure I can do it on the enum but fuck u and fuck this ok
pub fn get_response_type_from_heading(
    heading: String,
) -> Result<GamerResponseOption, ReactionsError> {
    for option in GamerResponseOption::VALUES.iter().copied() {
        if heading.contains(&option.heading()) {
            return Ok(option);
        }
    }
    return Err(ReactionsError::ParseHeadingError);
}
