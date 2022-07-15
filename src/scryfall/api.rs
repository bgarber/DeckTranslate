// Copyright 2022 Bryan Garber under GPLv3

use crate::scryfall::card;
use crate::scryfall::errors;

const SCRYFALL_API: &str = "https://api.scryfall.com/";
const CARDSEARCH: &str = "cards/search";

pub fn query(q: &str) -> Result<card::Card, errors::QueryError> {
    if let Ok(resp) = reqwest::blocking::get(SCRYFALL_API.to_owned() +
        CARDSEARCH + "?q=" + q) {
        re
    } else {
    }

    Ok(card)
}

