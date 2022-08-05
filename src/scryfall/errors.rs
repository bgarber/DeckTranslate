// Copyright 2022 Bryan Garber under GPLv3

#[derive(Debug)]
pub enum Error {
    CardNotFound,
    UnexpectedData,
    ClientError(reqwest::Error),
    HTTPError(reqwest::StatusCode),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CardNotFound => write!(f, "card not found"),
            Error::UnexpectedData => write!(f, "unexpected data returned from server"),
            Error::ClientError(e) => write!(f, "client error: {}", e),
            Error::HTTPError(c) => write!(f, "http error: {}", c),
        }
    }
}
