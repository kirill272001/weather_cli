use std::error::Error;
use std::fmt;
#[derive(Debug)]

pub enum AppError {
    UnknownCity(String),
    Http(reqwest::Error),
    Time(String),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UnknownCity(city) => {
                write!(
                    f,
                    "Місто '{}' не підтримується. Спробуй, наприклад: Kyiv, Lviv, London, Berlin.",
                    city
                )
            }
            AppError::Http(e) => {
                write!(f, "Проблема з мережею або API: {}", e)
            }
            AppError::Time(msg) => {
                write!(f, "Помилка сервісу часу: {}", msg)
            }
            AppError::Other(msg) => {
                write!(f, "Сталася невідома помилка: {}", msg)
            }
        }
    }
}

impl Error for AppError {}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Http(err)
    }
}
