// Copyright 2022 Bryan Garber under GPLv3

pub mod scryfall;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process;

fn translate_deck() {
    let filename = "tests/decksample.txt";
    let file = File::open(filename).unwrap_or_else(|err| {
        println!("error opening file: {err}");
        process::exit(0);
    });
    let file_reader = io::BufReader::new(file);

    for line in file_reader.lines() {
        if let Ok(l) = line {
            let card: scryfall::card::Card = l.into();
            println!("{card:?}");
        }
    }
}

fn main() {
    /*
    let pwd = std::env::var("PWD").unwrap();
    println!("pwd = {pwd}");
    */

    translate_deck();


    let card_list = scryfall::api::query("Ajani").unwrap();
    println!("returned cards: {:?}", card_list);

    let card = &card_list[0];
    println!("using card: {:?}", card);

    let tr_card = scryfall::api::find_card(&card.set, card.collector_number, "pt").unwrap();
    println!("translated card: {:?}", tr_card);
}
