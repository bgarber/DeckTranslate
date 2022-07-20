// Copyright 2022 Bryan Garber under GPLv3

use crate::scryfall::card::Card;
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
    let json_data = response.text().unwrap(); // hopeful unwrap. :)
    let parsed_json: serde_json::Value = serde_json::from_str(&json_data).unwrap();

    match &parsed_json["data"] {
        serde_json::Value::Array(card_list) => {
            let mut card_vec: Vec<Card> = Vec::new();
            for c in card_list {
                let card = Card::from(c);
                card_vec.push(card);
            }

            Ok(card_vec)
        },
        _ => Err(QueryError::UnexpectedData),
    }
}

pub fn find_card(code: &str, number: u32, lang: &str) -> Result<Card, QueryError> {
    let find_card_ep = format!("cards/{}/{}/{}", code, number, lang);
    let response = scrycall(&find_card_ep)?;
    let json_data = response.text().unwrap(); // hopeful unwrap. :)
    let parsed_json: serde_json::Value = serde_json::from_str(&json_data).unwrap();

    Ok(Card::from(&parsed_json))
}
