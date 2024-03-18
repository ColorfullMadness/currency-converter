use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;
use std::hash::Hash;

#[derive(Debug, Deserialize)]
struct Conversion{
    amount: f32,
    base: String,
    date: String,
    rates: HashMap<String, f32>
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let client = Client::new();

    let from = std::env::args().nth(1).expect("one arg");
    let to = std::env::args().nth(2).expect("two arg");
    let amount = std::env::args().nth(3).expect("three arg");
    println!("Conversion\n from: {from}\n to: {to}\n amount: {amount}");

    let conversion_rate_dto: Conversion = client.get("https://www.frankfurter.app/latest")
        .query(&[("from", from)])
        .query(&[("to", to.clone())])
        .send()
        .await?
        .json()
        .await?;

    let amount_p = amount.parse::<f32>().expect("Couldn't parse.");
    let conversion_rate = conversion_rate_dto.rates.get(&to).expect("Couldn't get conversion rate from payload");

    let converted = amount_p * conversion_rate;
    println!("Converted: {converted}");
    Ok(())
}
