use crate::util::{add_mention_to_response_list, strip_mention_from_response_lists};
use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed};
use serenity::model::interactions::message_component::{ButtonStyle, MessageComponentInteraction};
use serenity::prelude::*;
use std::str::FromStr;
use std::{fmt, vec};

pub enum GamerResponseOption {
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
    pub fn emoji(&self) -> char {
        match self {
            Self::Yes => '✅',
            Self::No => '❌',
            Self::Maybe => '❔',
            Self::Late => '⌛',
        }
    }

    pub fn heading(&self) -> String {
        match self {
            Self::Yes => format!("{} Gamers", self.emoji()),
            Self::No => format!("{} Rats", self.emoji()),
            Self::Maybe => format!("{} Potential Gamers", self.emoji()),
            Self::Late => format!("{} Late Gamers", self.emoji()),
        }
    }

    // The index of the embed's fields array that matches this reaction type
    pub fn field_index(&self) -> usize {
        match self {
            Self::Yes => 0,
            Self::No => 1,
            Self::Maybe => 2,
            Self::Late => 3,
        }
    }

    pub fn button(&self) -> CreateButton {
        let mut b = CreateButton::default();
        b.custom_id(self.to_string().to_ascii_lowercase());
        b.emoji(self.emoji());
        b.label(self);
        b.style(ButtonStyle::Primary);
        b
    }

    pub fn action_row() -> CreateActionRow {
        let mut ar = CreateActionRow::default();
        // We can add up to 5 buttons per action row
        ar.add_button(GamerResponseOption::Yes.button());
        ar.add_button(GamerResponseOption::No.button());
        ar.add_button(GamerResponseOption::Maybe.button());
        ar.add_button(GamerResponseOption::Late.button());
        ar
    }
}

impl FromStr for GamerResponseOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(GamerResponseOption::Yes),
            "no" => Ok(GamerResponseOption::No),
            "maybe" => Ok(GamerResponseOption::Maybe),
            "late" => Ok(GamerResponseOption::Late),
            _ => Err(()),
        }
    }
}

pub async fn handle_lobby_reaction(
    ctx: &Context,
    reaction: MessageComponentInteraction,
    option: GamerResponseOption,
) {
    let mut existing_embed = reaction.message.embeds[0].clone();
    let existing_fields = existing_embed.fields.clone();
    existing_embed.fields = vec![];
    let mut message = reaction.message;

    // User doesn't exist in this list
    if !existing_fields[option.field_index()]
        .value
        .contains(&reaction.user.id.to_string())
    {
        // let user_mention = Mention::User(reaction.user.id);
        let stripped_data =
            strip_mention_from_response_lists(existing_fields.clone(), reaction.user.id).await;
        let data_with_new_user =
            add_mention_to_response_list(stripped_data, option, reaction.user.id).await;

        // replace the new_data with the updated one
        let _update_result = message
            .edit(&ctx.http, |f| {
                f.embed(|e| {
                    *e = CreateEmbed::from(existing_embed);
                    e.fields(data_with_new_user)
                        .thumbnail("attachment://jonadello.png")
                })
            })
            .await;
    }
}
