use std::collections::HashMap;
use reqwest::{Client, Error};
use serde::Deserialize;
use std::hash::Hash;
use std::io::stdout;
use std::num::ParseFloatError;
use std::ops::Index;
use chrono::{Days, Months};
use derive_more::{Display, Error};
use clap::{ArgAction, Parser};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::{Rect, Style, Stylize};
use ratatui::{symbols, Terminal};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType};
use chrono::prelude::*;
use ordered_float::OrderedFloat;

#[derive(Debug, Deserialize)]
struct Conversion {
    amount: f32,
    base: String,
    date: String,
    rates: HashMap<String, f32>,
}

#[derive(Debug, Deserialize)]
struct ConversionSeries {
    amount: f32,
    base: String,
    start_date: String,
    end_date: String,
    rates: HashMap<String, HashMap<String, f32>>,
}

#[derive(Debug, Display, Error)]
struct AppError {
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

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(after_help = "Simple currency converter.")]
struct Cli {
    #[arg(help = "Code of base currency.")]
    from_currency_code: Option<String>,

    #[arg(help = "Code of target currency.")]
    to_currency_code: Option<String>,

    #[arg(short = 'a', help = "Amount of base currency to be exchanged.", requires = "from_currency_code", requires="to_currency_code")]
    amount: Option<f32>,

    #[arg(short = 'l', long = "list", exclusive = true, default_missing_value = "true", default_value = "false", require_equals = true, num_args = 0..=1, action = ArgAction::Set)]
    #[arg(help = "Lists all of the available currency codes and their full names.")]
    list: bool,

    #[arg(short = 'c', long = "chart", requires="to_currency_code", requires="from_currency_code")]
    #[arg(default_missing_value = "true", default_value = "false", require_equals = true, num_args = 0..=1, action = ArgAction::Set)]
    #[arg(help = "Displays a chart of exchange rate from the last 30 days. Requires both currency codes. ")]
    chart: bool,

    #[arg(short = 'i', long = "invalidate_cache")]
    #[arg(default_missing_value = "true", default_value = "false", require_equals = true, num_args = 0..=1, action = ArgAction::Set)]
    #[arg(help = "Invalidate cached list of available currencies.")]
    invalidate: bool,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let client = Client::new();
    let parsed = Cli::parse();
    let from = parsed.from_currency_code.as_ref();
    let to = parsed.to_currency_code.as_ref();
    let amount = parsed.amount.as_ref();

    let response = client.get("https://www.frankfurter.app/currencies")
        .send()
        .await?;

    let currencies: &HashMap<String, String> = response.json().await?;

    if parsed.list {
        println!("Today's exchange rates for {}:", from.unwrap());
        currencies.iter().for_each(|(code, name)| {
            println!("\t {code} - {name}")
        });
        return Ok(());
    }

    if parsed.chart {
        let from = parsed.from_currency_code.as_ref().unwrap();
        let to = parsed.to_currency_code.as_ref().unwrap();

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let date = Local::now();
        let date_back = date - Months::new(1);
        let date_half_back = date - Days::new(15);

        let response = client.get(format!("https://www.frankfurter.app/{}..{}",
                                          date_back.format("%Y-%m-%d").to_string(),
                                          date.format("%Y-%m-%d").to_string()))
            .query(&[("from", from)])
            .query(&[("to", to)])
            .send()
            .await?;

        let series: ConversionSeries = response.json().await?;
        let data: Vec<(f64, f64)> = series.rates.iter().map(|(day, value)|{
            let day = NaiveDate::parse_from_str(day,"%Y-%m-%d").unwrap().and_hms_opt(0,0,0).unwrap();
            let day1 = DateTime::<Utc>::from_naive_utc_and_offset(day, Utc).timestamp() as f64;
            (day1, *value.index(to) as f64)
        })
            .collect();

        let datasets = vec![
            Dataset::default()
                .name("Exchange rate last 30 days")
                .marker(symbols::Marker::Dot)
                .graph_type(GraphType::Scatter)
                .style(Style::default().green())
                .data(data.as_slice())
        ];

        let keys_orderable: Vec<OrderedFloat<f64>> = data.iter().map(|(key, _)| OrderedFloat(*key)).collect();
        let values_orderable: Vec<OrderedFloat<f64>> = data.iter().map(|(_, value)| OrderedFloat(*value)).collect();

        let max_x = keys_orderable.iter().max().ok_or(AppError{message: "Couldnt get values".to_string()})?;
        let min_x = keys_orderable.iter().min().ok_or(AppError{message: "Couldnt get values".to_string()})?;
        let max_y = values_orderable.iter().max().ok_or(AppError{message: "Couldnt get values".to_string()})?;
        let min_y = values_orderable.iter().min().ok_or(AppError{message: "Couldnt get values".to_string()})?;

        let min_formatted = min_y.0.to_string();
        let min_split = min_formatted.split_at(6).0;
        let max_formatted = max_y.0.to_string();
        let max_split = max_formatted.split_at(6).0;

        let x_axis = Axis::default()
            .title("timestamp".red())
            .style(Style::default().white())
            .bounds([min_x.0, max_x.0])
            .labels(vec![date_back.format("%Y-%m-%d").to_string().into(),date_half_back.format("%Y-%m-%d").to_string().into(), date.format("%Y-%m-%d").to_string().into()]);

        let y_axis = Axis::default()
            .title("rate".red())
            .style(Style::default().white())
            .bounds([min_y.0, max_y.0])
            .labels(vec![min_split.into(), max_split.into()]);

        let chart = Chart::new(datasets)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .block(Block::default().title("Chart").borders(Borders::ALL));

        terminal.clear()?;
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(chart,Rect::new(area.left(), area.top(), area.width, area.height));
        })?;

        return Ok(());
    }

    if from.is_none() || !validate_currency_code(from.unwrap(), currencies){
        println!("\tFROM_CURRENCY_CODE must be present and a valid code.");
        println!("\tUse --help to get more information.");
        return Ok(());
    }

    if to.is_none() && parsed.amount.is_none() {
        let from = from.unwrap();
        let response = client.get("https://www.frankfurter.app/latest")
            .query(&[("from", from.clone())])
            .send()
            .await?;

        let rates: Conversion = response.json().await?;

        println!("Today's exchange rates for {from}:");
        rates.rates.iter().for_each(|(code, rate)| println!("\t {code} - {rate}"));
    } else if to.is_some() && validate_currency_code(to.unwrap(), currencies) && amount.is_none() {
        let from = from.unwrap();
        let to = to.unwrap();

        let response = client.get("https://www.frankfurter.app/latest")
            .query(&[("from", from)])
            .query(&[("to", to)])
            .send()
            .await?;

        let body: Conversion = response.json().await?;
        let conversion_rate = body.rates.get(to).unwrap();

        println!("Conversion rate from: {from} to: {to} is: {conversion_rate}");
    } else if parsed.to_currency_code.is_some() && parsed.amount.is_some() {
        let from = parsed.from_currency_code.as_ref().unwrap();
        let to = parsed.to_currency_code.as_ref().unwrap();
        let amount = parsed.amount.as_ref().unwrap();

        let response = client.get("https://www.frankfurter.app/latest")
            .query(&[("from", from)])
            .query(&[("to", to)])
            .send()
            .await?;

        let body: Conversion = response.json().await?;
        let conversion_rate = body.rates.get(to).unwrap();
        let exchanged = conversion_rate * amount;

        println!("{amount} {from} at {conversion_rate} exchange rate is worth {exchanged} {to}");
    } else {
        return Err(AppError{
            message: "Incorrect state, please contact system administrator.".to_string()
        });
    }
    return Ok(());
}

fn validate_currency_code(code: &String, codes: &HashMap<String, String>) -> bool {
    codes.keys().any(|key| key.eq(code))
}
