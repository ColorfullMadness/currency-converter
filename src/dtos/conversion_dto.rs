use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConversionDTO {
    pub amount: f32,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f32>,
}

