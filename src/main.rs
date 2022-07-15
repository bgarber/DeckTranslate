// Copyright 2022 Bryan Garber under GPLv3

pub mod scryfall;

fn main() {
    if let Ok(card) = scryfall::api::query("Ajani Goldmane") {
        println!("returned card: {:?}", card);
    }
}
