// Copyright 2022 Bryan Garber under GPLv3

use std::fs::File;
use std::io;
use std::io::BufRead;

use crate::scryfall;

#[derive(Debug)]
pub enum Error {
    OpenError(std::io::Error),
    DeckParseError,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::OpenError(e)
    }
}

pub struct DeckItem {
    copies: u8,
    card: scryfall::card::Card,
}

impl std::fmt::Display for DeckItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} ({}) {}",
            self.copies,
            self.card.name(),
            self.card.set().to_uppercase(),
            self.card.number(),
        )
    }
}

// Implement type conversion from String to a Card
impl std::str::FromStr for DeckItem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"(\d+) ([\w,\-' /]+) \((\w{3,4})\) (\d+)").unwrap();

        if let Some(caps) = re.captures(&s) {
            let deck_item_copies: u8 = caps.get(1).unwrap().as_str().parse().unwrap();
            let card_name = caps.get(2).unwrap().as_str();
            let card_set = caps.get(3).unwrap().as_str().to_lowercase();
            let card_number: u32 = caps.get(4).unwrap().as_str().parse().unwrap();

            Ok(DeckItem {
                copies: deck_item_copies,
                card: scryfall::card::Card::new(
                    String::from(card_name),
                    String::from(card_name),
                    String::from("en"), // fix-me, assuming English
                    String::from(card_set),
                    card_number,
                ),
            })
        } else {
            Err(Error::DeckParseError)
        }
    }
}

pub type Deck = Vec<DeckItem>;

/// Loads a deck listing from a file
pub fn load(filename: &str) -> Result<Deck, Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut deck = Deck::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let deck_item: DeckItem = l.parse()?;
            deck.push(deck_item);
        }
    }

    Ok(deck)
}

// Translates a deck listing to target language
pub fn translate(deck: Deck, target_lang: &str) -> Result<Deck, scryfall::errors::Error> {
    let mut translated_deck = Deck::new();

    for deck_item in deck {
        let card =
            scryfall::api::find_card(deck_item.card.set(), deck_item.card.number(), target_lang)?;
        let tr_item = DeckItem {
            copies: deck_item.copies,
            card: card,
        };

        translated_deck.push(tr_item);
    }

    Ok(translated_deck)
}
