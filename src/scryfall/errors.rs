// Copyright 2022 Bryan Garber under GPLv3

pub enum QueryError {
    CardNotFound,
    ClientError(reqwest::Error),
    HTTPError(reqwest::StatusCode),
}

impl ToString for QueryError {
    fn to_string(&self) -> String {
        match self {
            QueryError::CardNotFound => String::from("card not found"),
            QueryError::ClientError(e) => format!("client error: {}", e),
            QueryError::HTTPError(c) => format!("http error: {}", c),
        }
    }
}
