// Copyright 2022 Bryan Garber under GPLv3

pub mod scryfall;

fn main() {
    let card_list = scryfall::api::query("Ajani").unwrap();

    println!("returned cards: {:?}", card_list);

    let card = &card_list[0];
    println!("using card: {:?}", card);

    let tr_card =
        scryfall::api::find_card(&card.set, card.collector_number.parse().unwrap(), "pt").unwrap();
    println!("translated card: {:?}", tr_card);
}
