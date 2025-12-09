use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    UnknownCity(String),
    Http(reqwest::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UnknownCity(city) => write!(f, "Місто '{}' не відоме для сервісу часу", city),
            AppError::Http(e) => write!(f, "Помилка HTTP: {}", e),
        }
    }
}

impl Error for AppError {}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Http(err)
    }
}