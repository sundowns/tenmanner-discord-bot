use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
};
use serenity::model::misc::EmojiIdentifier;
use serenity::model::prelude::*;
use serenity::prelude::*;

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
