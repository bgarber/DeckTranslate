// Copyright 2022 Bryan Garber under GPLv3

pub mod deck;
pub mod scryfall;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// deck-translate is a CLI tool to quickly "translate" Magic: the Gathering
/// decks to a specified language.
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
        match deck::load(&deckfile) {
            Ok(deck) => {
                let translated = deck::translate(deck, &args.lang).unwrap();
                for card in translated {
                    println!("{card}");
                }
            }
            Err(e) => eprintln!("error: {:?}", e),
        }
    } else {
        eprintln!("deck-translate REPL not implemented... yet!");
    }

    //let card_list = scryfall::api::query("Ajani").unwrap();
    //println!("returned cards: {:?}", card_list);
}
