// Copyright 2022 Bryan Garber under GPLv3

pub mod scryfall;

fn main() {
    let query_resp = scryfall::api::query("Ajani Goldmane");
    match query_resp {
        Ok(card) => {
            println!("returned card: {:?}", card);
        },
        Err(err) => {
            println!("error: {}", err.to_string());
        },
    }
}
