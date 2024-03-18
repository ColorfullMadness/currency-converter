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
    println!("from: {from:?}, to: {to:?}, amount: {amount:?}");

    let conversion_rate: Conversion = client.get("https://www.frankfurter.app/latest")
        .query(&[("from", from)])
        .query(&[("to", to.clone())])
        .send()
        .await?
        .json()
        .await?;


    let converted = amount.parse::<f32>().expect("cant parse") * conversion_rate.rates.get(&to).expect("no rate");
    println!("Converted: {converted}");
    Ok(())
}
