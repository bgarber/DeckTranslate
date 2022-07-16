// Copyright 2022 Bryan Garber under GPLv3

use crate::scryfall::card::{Card, CardList};
use crate::scryfall::errors::QueryError;

const SCRYFALL_API: &str = "https://api.scryfall.com/";
const CARDSEARCH: &str = "cards/search";

pub fn query(q: &str) -> Result<Vec<Card>, QueryError> {
    let url = SCRYFALL_API.to_owned() + CARDSEARCH + "?q=" + q;
    let resp_result = reqwest::blocking::get(url);

    match resp_result {
        Ok(resp) => match resp.status() {
            reqwest::StatusCode::OK => match resp.json::<CardList>() {
                Ok(card_list) => {
                    println!("query returned {} cards", card_list.total_cards);
                    Ok(card_list.data)
                }
                Err(err) => Err(QueryError::ClientError(err)),
            },
            reqwest::StatusCode::NOT_FOUND => Err(QueryError::CardNotFound),
            _ => Err(QueryError::HTTPError(resp.status())),
        },
        Err(err) => Err(QueryError::ClientError(err)),
    }
}
