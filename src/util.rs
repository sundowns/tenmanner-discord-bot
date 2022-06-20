use std::vec;

use serenity::model::interactions::message_component::MessageComponentInteraction;
use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
};
use serenity::model::misc::EmojiIdentifier;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::reactions::GamerResponseOption;

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
    except_for: GamerResponseOption,
    mention: Mention,
) -> Vec<EmbedField> {
    // Loop over the data, for each collectio
    vec![EmbedField::new(
        "stub".to_string(),
        "stub".to_string(),
        false,
    )]
}
