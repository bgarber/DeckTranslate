// Copyright 2022 Bryan Garber under GPLv3

use crate::scryfall::card::{Card, CardList};
use crate::scryfall::errors::QueryError;

const SCRYFALL_API: &str = "https://api.scryfall.com/";

fn scrycall(endpoint: &String) -> Result<reqwest::blocking::Response, QueryError> {
    let url = format!("{}{}", SCRYFALL_API, endpoint);
    let result = reqwest::blocking::get(url);

    match result {
        Ok(resp) => match resp.status() {
            reqwest::StatusCode::OK => Ok(resp),
            reqwest::StatusCode::NOT_FOUND => Err(QueryError::CardNotFound),
            _ => Err(QueryError::HTTPError(resp.status())),
        },
        Err(err) => Err(QueryError::ClientError(err)),
    }
}

pub fn query(q: &str) -> Result<Vec<Card>, QueryError> {
    let query_ep = format!("cards/search?q={}", q);
    let response = scrycall(&query_ep)?;

    match response.json::<CardList>() {
        Ok(card_list) => {
            println!("query returned {} cards", card_list.total_cards);
            Ok(card_list.data)
        }
        Err(err) => Err(QueryError::ClientError(err)),
    }
}

pub fn find_card(code: &str, number: u32, lang: &str) -> Result<Card, QueryError> {
    let find_card_ep = format!("cards/{}/{}/{}", code, number, lang);
    let response = scrycall(&find_card_ep)?;

    match response.json::<Card>() {
        Ok(card) => Ok(card),
        Err(err) => Err(QueryError::ClientError(err)),
    }
}
