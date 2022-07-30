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
use crate::storage_manager::PostReactionsDto;
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

pub fn to_tuple(from: EmbedField) -> (String, String, bool) {
    (from.name, from.value, from.inline)
}

pub async fn update_message_embed(
    ctx: &Context,
    mut message: Message,
    response_data: PostReactionsDto,
    summary: LobbySignupSummary,
) -> Result<(), SerenityError> {
    let status = LobbyStatus::from(summary);
    let mut existing_embed = message.embeds[0].clone();
    existing_embed.fields = vec![];
    let embed_data = convert_response_data_to_embed_fields(response_data, summary);

    // replace the new_data with the updated one
    return message
        .edit(&ctx.http, |f| {
            f.embed(|e| {
                *e = CreateEmbed::from(existing_embed);
                e.colour(status.colour())
                    .fields(embed_data)
                    .thumbnail("attachment://jonadello.png")
            })
        })
        .await;
}

// converts the list of user ID values to a string with discord mentions
fn user_ids_to_mentions(user_ids: Vec<UserId>) -> String {
    if user_ids.len() == 0 {
        return String::from(DEFAULT_LIST_STRING);
    }
    let mut result: String = String::default();
    for user_id in user_ids {
        result.push_str(format!(" {}", Mention::from(user_id)).as_str());
    }
    result
}

fn convert_response_data_to_embed_fields(
    response_data: PostReactionsDto,
    summary: LobbySignupSummary,
) -> Vec<(String, String, bool)> {
    vec![
        (
            format!(
                "{}   [{}]",
                GamerResponseOption::Yes.heading(),
                summary.value_for_response_type(GamerResponseOption::Yes)
            ),
            user_ids_to_mentions(response_data.yes),
            false,
        ),
        (
            format!(
                "{}   [{}]",
                GamerResponseOption::Maybe.heading(),
                summary.value_for_response_type(GamerResponseOption::Maybe)
            ),
            user_ids_to_mentions(response_data.maybe),
            false,
        ),
        (
            format!(
                "{}   [{}]",
                GamerResponseOption::Late.heading(),
                summary.value_for_response_type(GamerResponseOption::Late)
            ),
            user_ids_to_mentions(response_data.late),
            false,
        ),
        (
            format!(
                "{}   [{}]",
                GamerResponseOption::No.heading(),
                summary.value_for_response_type(GamerResponseOption::No)
            ),
            user_ids_to_mentions(response_data.no),
            false,
        ),
    ]
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
