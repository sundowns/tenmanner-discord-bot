use crate::util::{add_mention_to_response_list, strip_mention_from_response_lists};
use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed};
use serenity::model::channel::Message;
use serenity::model::interactions::message_component::{ButtonStyle, MessageComponentInteraction};
use serenity::prelude::*;
use serenity::utils::Colour;
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

static FULL_LOBBY_COUNT: usize = 10;

struct LobbySignupSummary {
    yes: usize,
    maybe: usize,
    no: usize,
    late: usize,
}

impl Default for LobbySignupSummary {
    fn default() -> Self {
        LobbySignupSummary {
            yes: 0,
            maybe: 0,
            no: 0,
            late: 0,
        }
    }
}

enum LobbyStatus {
    Empty,
    Some,
    FullWithMaybe,
    FullYes,
}

impl LobbyStatus {
    pub fn colour(&self) -> Colour {
        match self {
            Self::Empty => 0xff0000,
            Self::Some => 0xff7700,
            Self::FullWithMaybe => 0xffcc00,
            Self::FullYes => 0x00ff66,
        }
    }
}

impl From<LobbySignupSummary> for LobbyStatus {
    fn from(summary: LobbySignupSummary) -> Self {
        if summary.yes >= FULL_LOBBY_COUNT {
            return LobbyStatus::FullYes;
        }
        if summary.yes + summary.maybe >= FULL_LOBBY_COUNT {
            return LobbyStatus::FullWithMaybe;
        }
        if summary.yes + summary.maybe > 0 {
            return LobbyStatus::Some;
        }
        return LobbyStatus::Empty;
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

// Count reactions in each field.
// Sum all reactions, compare to thresholds and update embed colour accordingly
pub async fn summarise_reactions(ctx: Context, message: Message) {
    let mut count: usize = 0;
    let mut summary = LobbySignupSummary::default();

    if let Some(mut existing_embed) = message.embeds.first() {
        for field in existing_embed.fields.clone().into_iter() {
            let count_for_field = field.value.split(" ").count();
            match field.name {
                GamerResponseOption::Yes.heading() => {}
                _ => {}
            };
            count += count_for_field;
        }

        // let status: LobbyStatus = LobbyStatus::from(LobbySignupSummary)
    }

    // count reactions in each column
}
