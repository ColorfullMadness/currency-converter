use std::num::ParseFloatError;
use derive_more::{Display, Error};
use reqwest::Error;

#[derive(Debug, Display, Error)]
pub struct AppError {
    pub message: String,
}

impl From<reqwest::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError {
            message: value.to_string(),
        }
    }
}

impl From<ParseFloatError> for AppError {
    fn from(value: ParseFloatError) -> Self {
        AppError {
            message: value.to_string()
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError {
            message: value.to_string()
        }
    }
}
