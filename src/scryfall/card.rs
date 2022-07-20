// Copyright 2022 Bryan Garber under GPLv3

// Card defines a card from the Scryfall database
#[derive(Debug)]
pub struct Card {
    pub name: String,
    pub printed_name: String,
    pub lang: String,
    pub set: String,
    pub collector_number: u32,
}

impl From<&serde_json::Value> for Card {
    fn from(c: &serde_json::Value) -> Self {
        Card {
            name: if let Some(name) = c["name"].as_str() {
                String::from(name)
            } else {
                String::from("")
            },
            lang: if let Some(lang) = c["lang"].as_str() {
                String::from(lang)
            } else {
                String::from("")
            },
            set: if let Some(set) = c["set"].as_str() {
                String::from(set)
            } else {
                String::from("")
            },
            printed_name: match c.get("printed_name") {
                Some(printed_name) => {
                    if let Some(pn) = printed_name.as_str() {
                        String::from(pn)
                    } else {
                        String::from("")
                    }
                },
                None => {
                    if let Some(pn) = c["name"].as_str() {
                        String::from(pn)
                    } else {
                        String::from("")
                    }
                },
            },
            collector_number: if let Some(coll_n) = c["collector_number"].as_str() {
                match String::from(coll_n).parse::<u32>() {
                    Ok(cn) => cn,
                    Err(_) => 0,
                }
            } else {
                0
            },
        }
    }
}

