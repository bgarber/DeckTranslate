// Copyright 2022 Bryan Garber under GPLv3

// Macro for extracting a String from a Option<Value>
macro_rules! extract_string {
    ($value:expr) => {
        if let Some(v) = $value {
            String::from(v)
        } else {
            String::from("")
        }
    };
}

// Card defines a card from the Scryfall database
#[derive(Debug)]
pub struct Card {
    name: String,
    printed_name: String,
    lang: String,
    set: String,
    collector_number: u32,
}

// Method implementations for a Card object
impl Card {
    pub fn name(&self) -> &str {
        self.printed_name.as_str()
    }

    pub fn set(&self) -> &str {
        self.set.as_str()
    }

    pub fn number(&self) -> u32 {
        self.collector_number
    }
}

// Implement type conversion from serde_json::Value to a Card
impl From<&serde_json::Value> for Card {
    fn from(c: &serde_json::Value) -> Self {
        Card {
            name: extract_string!(c["name"].as_str()),
            lang: extract_string!(c["lang"].as_str()),
            set: extract_string!(c["set"].as_str()),
            printed_name: match c.get("printed_name") {
                Some(printed_name) => extract_string!(printed_name.as_str()),
                None => {
                    if let Some(serde_json::Value::Array(card_faces)) = c.get("card_faces") {
                        let face = &card_faces[0];
                        if let Some(pn) = face.get("printed_name") {
                            extract_string!(pn.as_str())
                        } else {
                            extract_string!(face["name"].as_str())
                        }
                    } else {
                        extract_string!(c["name"].as_str())
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

// Implement type conversion from String to a Card
impl std::str::FromStr for Card {
    type Err = crate::scryfall::errors::QueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"(\d+) ([\w,\-' /]+) \((\w{3,4})\) (\d+)").unwrap();

        if let Some(caps) = re.captures(&s) {
            let card_name = caps.get(2).unwrap().as_str();
            let card_set = caps.get(3).unwrap().as_str().to_lowercase();
            let card_number: u32 = caps.get(4).unwrap().as_str().parse().unwrap();

            Ok(Card {
                name: String::from(card_name),
                printed_name: String::from(card_name),
                lang: String::from("en"), // fix-me, assuming English
                set: String::from(card_set),
                collector_number: card_number,
            })
        } else {
            Ok(Card {
                name: String::from(""),
                printed_name: String::from(""),
                lang: String::from(""),
                set: String::from(""),
                collector_number: 0,
            })
        }
    }
}

