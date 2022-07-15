// Copyright 2022 Bryan Garber under GPLv3

pub enum QueryError {
    CardNotFound,
    Timeout,
    ServerError,
}

impl QueryError {
    pub fn to_str(&self) -> &str {
        match self {
            QueryError::CardNotFound => "card not found",
            QueryError::Timeout => "timeout waiting response",
            QueryError::ServerError => "general error on server",
        }
    }
}

impl ToString for QueryError {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}
