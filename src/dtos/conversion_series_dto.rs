use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConversionSeriesDTO {
    pub amount: f32,
    pub base: String,
    pub start_date: String,
    pub end_date: String,
    pub rates: HashMap<String, HashMap<String, f32>>,
}