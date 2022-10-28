// Copyright 2022 Bryan Garber under GPLv3

pub mod card;
pub mod errors;

pub type Result<T> = core::result::Result<T, errors::Error>;

// Consider making a Scryfall client struct...
const SCRYFALL_API: &str = "https://api.scryfall.com/";
const REQUESTS_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);
static mut LAST_API_CALL: std::time::SystemTime = std::time::SystemTime::UNIX_EPOCH;

fn scrycall(endpoint: &String) -> Result<reqwest::blocking::Response> {
    // I barely learned Rust and I'm already using unsafe code.
    // Shame on me! XD
    unsafe {
        // The Scryfall RESTAPI kindly asks any client to keep a sane number of
        // requests. So we insert a sleep of 50 milliseconds between each call.
        let current_time = std::time::SystemTime::now();
        let expected_time = LAST_API_CALL + REQUESTS_INTERVAL;
        if let Err(err) = current_time.duration_since(expected_time) {
            // expected_time is later than current_time, then err will have the
            // Duration up to expected_time
            std::thread::sleep(err.duration());
        }
    }

    let url = format!("{}{}", SCRYFALL_API, endpoint);
    let result = reqwest::blocking::get(url);

    match result {
        Ok(resp) => match resp.status() {
            reqwest::StatusCode::OK => Ok(resp),
            reqwest::StatusCode::NOT_FOUND => Err(errors::Error::CardNotFound),
            _ => Err(errors::Error::HTTPError(resp.status())),
        },
        Err(err) => Err(errors::Error::ClientError(err)),
    }
}

pub fn query(q: &str) -> Result<card::CardList> {
    let query_ep = format!("cards/search?q={}", q);
    let response = scrycall(&query_ep)?;
    let json_data = response.text()?;

    let card_list: card::CardList = serde_json::from_str(&json_data)?;
    Ok(card_list)
}

pub fn find_card(code: &str, number: u32, lang: &str) -> Result<card::Card> {
    let find_card_ep = format!("cards/{}/{}/{}", code, number, lang);
    let response = scrycall(&find_card_ep)?;
    let json_data = response.text()?;

    let card: card::Card = serde_json::from_str(&json_data)?;
    Ok(card)
}
