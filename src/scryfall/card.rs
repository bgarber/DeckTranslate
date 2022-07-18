// Copyright 2022 Bryan Garber under GPLv3

use serde::{Serialize, Deserialize};

// Card defines a card from the Scryfall database
#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub name: String,
    pub printed_name: String,
    pub lang: String,
    pub set: String,
    pub collector_number: String,
}

// CardList defines a list of cards the Scryfall API may return
#[derive(Serialize, Deserialize, Debug)]
pub struct CardList {
    pub total_cards: u32,
    pub data: Vec::<Card>,
}
