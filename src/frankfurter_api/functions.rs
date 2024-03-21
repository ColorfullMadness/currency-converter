use std::collections::HashMap;
use chrono::{DateTime, Local};
use reqwest::Client;
use crate::dtos::conversion_dto::ConversionDTO;
use crate::dtos::conversion_series_dto::ConversionSeriesDTO;
use crate::errors::app_error::AppError;

pub async fn get_currencies(client: &Client) -> Result<HashMap<String, String>, AppError> {
    let response = client.get("https://www.frankfurter.app/currencies")
        .send()
        .await?;

    let currencies: HashMap<String, String> = response.json().await?;
    Ok(currencies)
}

pub async fn get_rates_between_dates(
    date_back: &DateTime<Local>,
    date: &DateTime<Local>,
    from: &String,
    to: &String,
    client: &Client,
) -> Result<ConversionSeriesDTO, AppError> {
    let url = format!("https://www.frankfurter.app/{}..{}",
                      date_back.format("%Y-%m-%d").to_string(),
                      date.format("%Y-%m-%d").to_string());
    let response = client.get(url)
        .query(&[("from", from)])
        .query(&[("to", to)])
        .send()
        .await?;

    let series: ConversionSeriesDTO = response.json().await?;
    Ok(series)
}

pub async fn get_rates(from: &String, client: &Client) -> Result<ConversionDTO, AppError> {
    let response = client.get("https://www.frankfurter.app/latest")
        .query(&[("from", from.clone())])
        .send()
        .await?;

    let rates: ConversionDTO = response.json().await?;
    Ok(rates)
}

pub async fn get_rate(from: &String, to: &String, client: &Client) -> Result<f32, AppError> {
    let response = client.get("https://www.frankfurter.app/latest")
        .query(&[("from", from)])
        .query(&[("to", to)])
        .send()
        .await?;

    let body: ConversionDTO = response.json().await?;
    let conversion_rate = body.rates.get(to).unwrap();
    Ok(*conversion_rate)
}