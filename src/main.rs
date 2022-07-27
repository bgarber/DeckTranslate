// Copyright 2022 Bryan Garber under GPLv3

pub mod scryfall;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process;

// Begins the translation of a deckfile
fn translate_deck(deck: &str) {
    let file = File::open(deck).unwrap_or_else(|err| {
        println!("error opening file: {err}");
        process::exit(0);
    });
    let file_reader = io::BufReader::new(file);

    for line in file_reader.lines() {
        if let Ok(l) = line {
            //let card: scryfall::card::Card = l.into();
            let card: scryfall::card::Card = l.parse().unwrap();
            let translated = scryfall::api::find_card(card.set(), card.number(), "pt").unwrap();

            println!("{} => {}", card.name(), translated.name());
        }
    }
}

fn main() {
    /*
    let pwd = std::env::var("PWD").unwrap();
    println!("pwd = {pwd}");
    */

    translate_deck("tests/decksample.txt");

    //let card_list = scryfall::api::query("Ajani").unwrap();
    //println!("returned cards: {:?}", card_list);
}
