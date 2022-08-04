// Copyright 2022 Bryan Garber under GPLv3

pub mod deck;
pub mod scryfall;

fn main() {
    /*
    let pwd = std::env::var("PWD").unwrap();
    println!("pwd = {pwd}");
    */

    match deck::load("tests/decksample.txt") {
        Ok(deck) => deck::translate(deck),
        Err(e) => println!("error: {:?}", e),
    }

    //let card_list = scryfall::api::query("Ajani").unwrap();
    //println!("returned cards: {:?}", card_list);
}
