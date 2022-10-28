// Copyright 2022 Bryan Garber under GPLv3

use serde::{Deserialize, Serialize};

// CardFace defines a face for modal cards
#[derive(Debug, Serialize, Deserialize)]
pub struct CardFace {
    name: String,
    printed_name: Option<String>,
}

impl CardFace {
    fn name(&self) -> &str {
        if let Some(pn) = &self.printed_name {
            pn.as_str()
        } else {
            self.name.as_str()
        }
    }
}

// Card defines a card from the Scryfall database
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    name: String,
    printed_name: Option<String>,
    lang: String,
    set: String,
    collector_number: String,
    card_faces: Option<Vec<CardFace>>,
}

// Method implementations for a Card object
impl Card {
    pub fn new(
        name: String,
        printed_name: Option<String>,
        lang: String,
        set: String,
        number: u32,
    ) -> Card {
        Card {
            name,
            printed_name,
            lang,
            set,
            collector_number: format!("{}", number),
            card_faces: None,
        }
    }

    pub fn name(&self) -> &str {
        if let Some(pn) = &self.printed_name {
            pn.as_str()
        } else if let Some(cf) = &self.card_faces {
            cf[0].name()
        } else {
            self.name.as_str()
        }
    }

    pub fn lang(&self) -> &str {
        self.lang.as_str()
    }

    pub fn set(&self) -> &str {
        self.set.as_str()
    }

    pub fn number(&self) -> u32 {
        // Since this is a private field, and I control its content, I can
        // safely assume the unwrap() will always succeed.
        self.collector_number.parse::<u32>().unwrap()
    }
}

// CardList defines a listing of cards, when querying the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct CardList {
    total_cards: u64,
    has_more: bool,
    next_page: Option<String>,
    data: Vec<Card>,
}
