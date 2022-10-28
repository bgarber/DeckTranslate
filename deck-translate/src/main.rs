// Copyright 2022 Bryan Garber under GPLv3

pub mod deck;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Bryan Garber <spellcasterbryan@gmail.com>")]
#[clap(version = "")]
#[clap(about = "Translate deck listings")]
#[clap(long_about = "deck-translate is a CLI tool to quickly \"translate\" \
    Magic: the Gathering decks to a specified language.")]
struct DeckTranslate {
    /// A file containing a valid deck listing
    #[clap(value_parser, value_name = "DECK")]
    deckfile: Option<String>,

    /// The target language to translate the deck listing to
    #[clap(short, long, value_parser)]
    lang: String,
}

fn main() {
    /*
    let pwd = std::env::var("PWD").unwrap();
    println!("pwd = {pwd}");
    */

    let args = DeckTranslate::parse();
    if let Some(deckfile) = args.deckfile {
        let deck_listing = deck::load(&deckfile).expect("could not open file");

        let translated_deck =
            deck::translate(deck_listing, &args.lang).expect("error translating deck");

        for card in translated_deck {
            println!("{card}");
        }
    } else {
        eprintln!("deck-translate REPL not implemented... yet!");
    }

    /*
    let card_list = scryfall::api::query("Ajani").unwrap();
    println!("returned cards: {:?}", card_list);
    */
}
