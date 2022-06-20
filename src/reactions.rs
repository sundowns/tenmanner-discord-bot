use crate::util::strip_mention_from_response_lists;
use crate::DEFAULT_LIST_STRING;
use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed};
use serenity::model::interactions::message_component::{ButtonStyle, MessageComponentInteraction};
use serenity::model::mention::Mention;
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

pub async fn handle_yes_reaction(ctx: &Context, reaction: MessageComponentInteraction) {
    let mut existing_embed = reaction.message.embeds[0].clone();
    let existing_fields = existing_embed.fields.clone();
    existing_embed.fields = vec![];
    let mut message = reaction.message;

    // User doesn't exist in this list
    if !existing_fields[0]
        .value
        .contains(&reaction.user.id.to_string())
    {
        let user_mention = Mention::User(reaction.user.id);
        let new_data = strip_mention_from_response_lists(
            existing_fields.clone(),
            GamerResponseOption::Yes,
            user_mention,
        );

        // TODO: replace the new_data with the updated one
        let _update_result = message
            .edit(&ctx.http, |f| {
                f.embed(|e| {
                    *e = CreateEmbed::from(existing_embed);
                    e.fields(vec![(
                        GamerResponseOption::Yes.heading(),
                        if existing_fields[0].value == DEFAULT_LIST_STRING {
                            user_mention.to_string()
                        } else {
                            format!("{} {}", existing_fields[0].value, user_mention)
                        },
                        false,
                    )])
                    .thumbnail("attachment://jonadello.png")
                })
            })
            .await;
    }
}

pub async fn handle_no_reaction(ctx: &Context, reaction: MessageComponentInteraction) {
    let mut existing_embed = reaction.message.embeds[0].clone();
    let existing_fields = existing_embed.fields.clone();
    existing_embed.fields = vec![];
    let mut message = reaction.message;

    // User doesn't exist in this list
    if !existing_fields[0]
        .value
        .contains(&reaction.user.id.to_string())
    {
        let _update_result = message
            .edit(&ctx.http, |f| {
                f.embed(|e| {
                    *e = CreateEmbed::from(existing_embed);
                    e.fields(vec![(
                        GamerResponseOption::No.heading(),
                        if existing_fields[1].value == DEFAULT_LIST_STRING {
                            Mention::User(reaction.user.id).to_string()
                        } else {
                            format!(
                                "{} {}",
                                existing_fields[1].value,
                                Mention::User(reaction.user.id)
                            )
                        },
                        false,
                    )])
                    .thumbnail("attachment://jonadello.png")
                })
            })
            .await;
    }
}

pub async fn handle_maybe_reaction(_ctx: &Context, _reaction: MessageComponentInteraction) {}

pub async fn handle_late_reaction(_ctx: &Context, _reaction: MessageComponentInteraction) {}
