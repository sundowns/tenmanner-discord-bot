use crate::util::{
    add_mention_to_response_list, get_response_type_from_heading, strip_mention_from_response_lists,
};
use crate::DEFAULT_LIST_STRING;
use serenity::builder::{CreateActionRow, CreateButton};
use serenity::model::interactions::message_component::{ButtonStyle, MessageComponentInteraction};
use serenity::utils::Colour;
use std::{error::Error, fmt, str::FromStr, vec};

#[derive(Debug)]
pub enum ReactionsError {
    ParseHeadingError,
    NoUpdateRequired,
}

impl ReactionsError {
    pub fn message(&self) -> &str {
        match self {
            Self::NoUpdateRequired => "User already responded, no change in data",
            Self::ParseHeadingError => "Failed to parse reaction group heading",
        }
    }
}

impl fmt::Display for ReactionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReactionsError: {}", self.message())
    }
}
impl Error for ReactionsError {}

#[derive(Clone, Copy)]
pub enum GamerResponseOption {
    Yes,
    Maybe,
    Late,
    No,
}

impl fmt::Display for GamerResponseOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yes => write!(f, "Yes"),
            Self::Maybe => write!(f, "Maybe"),
            Self::Late => write!(f, "Late"),
            Self::No => write!(f, "No"),
        }
    }
}

impl GamerResponseOption {
    pub const VALUES: [Self; 4] = [Self::Yes, Self::Maybe, Self::No, Self::Late];

    pub fn emoji(&self) -> char {
        match self {
            Self::Yes => '✅',
            Self::Maybe => '❔',
            Self::Late => '⌛',
            Self::No => '❌',
        }
    }

    pub fn heading(&self) -> String {
        match self {
            Self::Yes => format!("{} Gamers", self.emoji()),
            Self::Maybe => format!("{} Potential Gamers", self.emoji()),
            Self::Late => format!("{} Late Gamers", self.emoji()),
            Self::No => format!("{} Rats", self.emoji()),
        }
    }

    // The index of the embed's fields array that matches this reaction type
    pub fn field_index(&self) -> usize {
        match self {
            Self::Yes => 0,
            Self::Maybe => 1,
            Self::Late => 2,
            Self::No => 3,
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
        ar.add_button(GamerResponseOption::Maybe.button());
        ar.add_button(GamerResponseOption::Late.button());
        ar.add_button(GamerResponseOption::No.button());
        ar
    }
}

impl FromStr for GamerResponseOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(GamerResponseOption::Yes),
            "maybe" => Ok(GamerResponseOption::Maybe),
            "late" => Ok(GamerResponseOption::Late),
            "no" => Ok(GamerResponseOption::No),
            _ => Err(()),
        }
    }
}

static FULL_LOBBY_COUNT: usize = 10;

#[derive(Debug, Clone, Copy)]
pub struct LobbySignupSummary {
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

impl LobbySignupSummary {
    pub fn value_for_response_type(&self, response_type: GamerResponseOption) -> usize {
        match response_type {
            GamerResponseOption::Yes => self.yes,
            GamerResponseOption::Maybe => self.maybe,
            GamerResponseOption::No => self.no,
            GamerResponseOption::Late => self.late,
        }
    }
}

pub enum LobbyStatus {
    Empty,
    Some,
    FullWithMaybe,
    FullYes,
}

impl LobbyStatus {
    pub fn colour(&self) -> Colour {
        match self {
            Self::Empty => Colour::new(0xff0000),
            Self::Some => Colour::new(0xff7700),
            Self::FullWithMaybe => Colour::new(0xffcc00),
            Self::FullYes => Colour::new(0x00ff66),
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

impl fmt::Display for LobbyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FullYes => write!(f, "Full"),
            Self::FullWithMaybe => write!(f, "Full including maybes"),
            Self::Some => write!(f, "Some players"),
            Self::Empty => write!(f, "Empty"),
        }
    }
}

pub async fn build_reaction_data(
    reaction: MessageComponentInteraction,
    option: GamerResponseOption,
) -> Result<Vec<(String, String, bool)>, ReactionsError> {
    let mut existing_embed = reaction.message.embeds[0].clone();
    let existing_fields = existing_embed.fields.clone();
    existing_embed.fields = vec![];
    // let mut message = reaction.message;

    // User doesn't exist in this list
    if !existing_fields[option.field_index()]
        .value
        .contains(&reaction.user.id.to_string())
    {
        let stripped_data =
            strip_mention_from_response_lists(existing_fields.clone(), reaction.user.id).await;
        let data_with_new_user =
            add_mention_to_response_list(stripped_data, option, reaction.user.id).await;

        return Ok(data_with_new_user);
    }
    return Err(ReactionsError::NoUpdateRequired);
}

// Count reactions in each field.
// Sum all reactions, compare to thresholds and update embed colour accordingly
pub async fn summarise_reactions(
    reactions: Vec<(String, String, bool)>,
) -> Result<LobbySignupSummary, ReactionsError> {
    let mut summary = LobbySignupSummary::default();

    for (name, value, _inline) in reactions.into_iter() {
        let count_for_field = if value == DEFAULT_LIST_STRING {
            0
        } else {
            value.matches('@').count()
        };
        if let Ok(option_type) = get_response_type_from_heading(name) {
            match option_type {
                GamerResponseOption::Yes => {
                    summary.yes = count_for_field;
                }
                GamerResponseOption::Maybe => {
                    summary.maybe = count_for_field;
                }
                GamerResponseOption::Late => {
                    summary.late = count_for_field;
                }
                GamerResponseOption::No => {
                    summary.no = count_for_field;
                }
            }
        } else {
            return Err(ReactionsError::ParseHeadingError);
        }
    }
    return Ok(summary);
}
