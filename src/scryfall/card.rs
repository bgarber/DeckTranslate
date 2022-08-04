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
    pub fn new(
        name: String,
        printed_name: String,
        lang: String,
        set: String,
        collector_number: u32,
    ) -> Card {
        Card {
            name,
            printed_name,
            lang,
            set,
            collector_number,
        }
    }

    pub fn name(&self) -> &str {
        if self.printed_name.is_empty() {
            self.name.as_str()
        } else {
            self.printed_name.as_str()
        }
    }

    pub fn lang(&self) -> &str {
        self.lang.as_str()
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
                            String::from("")
                        }
                    } else {
                        String::from("")
                    }
                }
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
