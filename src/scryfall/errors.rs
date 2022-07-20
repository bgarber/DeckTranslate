// Copyright 2022 Bryan Garber under GPLv3

#[derive(Debug)]
pub enum QueryError {
    CardNotFound,
    UnexpectedData,
    ClientError(reqwest::Error),
    HTTPError(reqwest::StatusCode),
}

impl ToString for QueryError {
    fn to_string(&self) -> String {
        match self {
            QueryError::CardNotFound => String::from("card not found"),
            QueryError::UnexpectedData => String::from("unexpected data returned from server"),
            QueryError::ClientError(e) => format!("client error: {}", e),
            QueryError::HTTPError(c) => format!("http error: {}", c),
        }
    }
}
