mod dtos;
mod errors;
mod input;
mod frankfurter_api;

use std::collections::HashMap;
use reqwest::Client;
use std::io::stdout;
use std::ops::Index;
use chrono::{Days, Months};
use clap::Parser;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::{Rect, Style, Stylize};
use ratatui::{symbols, Terminal};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType};
use chrono::prelude::*;
use ordered_float::OrderedFloat;
use dtos::conversion_series_dto::ConversionSeriesDTO;
use errors::app_error::AppError;
use input::cli_parser::Cli;
use frankfurter_api::functions::{get_currencies, get_rates_between_dates, get_rates, get_rate };


#[tokio::main]
async fn main() -> Result<(), AppError> {
    let client = Client::new();
    let parsed = Cli::parse();
    let from = parsed.from_currency_code.as_ref();
    let to = parsed.to_currency_code.as_ref();
    let amount = parsed.amount.as_ref();

    let currencies = &get_currencies(&client).await?;

    if parsed.list {
        currencies.iter().for_each(|(code, name)| {
            println!("\t {code} - {name}")
        });
        return Ok(());
    }

    if parsed.chart {
        let from = parsed.from_currency_code.as_ref().unwrap();
        let to = parsed.to_currency_code.as_ref().unwrap();
        
        print_chart(from, to, &client).await?;
        return Ok(());
    }

    if from.is_none() || !validate_currency_code(from.unwrap(), currencies) {
        println!("\tFROM_CURRENCY_CODE must be present and a valid code.");
        println!("\tUse --help to get more information.");
        return Ok(());
    }

    if to.is_none() && parsed.amount.is_none() {
        let from = from.unwrap();

        let rates = get_rates(from, &client).await?;
        println!("Today's exchange rates for {from}:");
        rates.rates.iter().for_each(|(code, rate)| println!("\t {code} - {rate}"));
    } else if to.is_some() && validate_currency_code(to.unwrap(), currencies) && amount.is_none() {
        let from = from.unwrap();
        let to = to.unwrap();

        let conversion_rate = get_rate(from, to, &client).await?;
        println!("Conversion rate from: {from} to: {to} is: {conversion_rate}");
    } else if parsed.to_currency_code.is_some() && parsed.amount.is_some() {
        let from = parsed.from_currency_code.as_ref().unwrap();
        let to = parsed.to_currency_code.as_ref().unwrap();
        let amount = parsed.amount.as_ref().unwrap();

        let conversion_rate = get_rate(from, to, &client).await?;
        let exchanged = conversion_rate * amount;
        println!("{amount} {from} at {conversion_rate} exchange rate is worth {exchanged} {to}");
    } else {
        return Err(AppError {
            message: "Incorrect state, please contact system administrator.".to_string()
        });
    }
    return Ok(());
}

fn validate_currency_code(code: &String, codes: &HashMap<String, String>) -> bool {
    codes.keys().any(|key| key.eq(code))
}

async fn print_chart(from: &String, to: &String, client: &Client) -> Result<(), AppError> {
    let date = Local::now();
    let date_back = date - Months::new(1);
    let date_half_back = date - Days::new(15);

    let series = get_rates_between_dates(&date_back, &date, from, to, &client).await?;
    let data = process_series(series, to);
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

    let max_x = keys_orderable.iter().max().ok_or(AppError { message: "Couldnt get values".to_string() })?;
    let min_x = keys_orderable.iter().min().ok_or(AppError { message: "Couldnt get values".to_string() })?;
    let max_y = values_orderable.iter().max().ok_or(AppError { message: "Couldnt get values".to_string() })?;
    let min_y = values_orderable.iter().min().ok_or(AppError { message: "Couldnt get values".to_string() })?;

    let min_formatted = min_y.0.to_string();
    let min_split = min_formatted.split_at(6).0;
    let max_formatted = max_y.0.to_string();
    let max_split = max_formatted.split_at(6).0;

    let x_axis = Axis::default()
        .title("timestamp".red())
        .style(Style::default().white())
        .bounds([min_x.0, max_x.0])
        .labels(vec![date_back.format("%Y-%m-%d").to_string().into(),
                     date_half_back.format("%Y-%m-%d").to_string().into(),
                     date.format("%Y-%m-%d").to_string().into(),
        ]);
    let y_axis = Axis::default()
        .title("rate".red())
        .style(Style::default().white())
        .bounds([min_y.0, max_y.0])
        .labels(vec![min_split.into(), max_split.into()]);

    let chart = Chart::new(datasets)
        .x_axis(x_axis)
        .y_axis(y_axis)
        .block(Block::default().title("Chart").borders(Borders::ALL));

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    terminal.draw(|frame| {
        let area = frame.size();
        frame.render_widget(chart, Rect::new(area.left(), area.top(), area.width, area.height));
    })?;
    Ok(())
}

fn process_series(series: ConversionSeriesDTO, to: &String) -> Vec<(f64, f64)> {
    series.rates.iter().map(|(day, value)| {
        let day = NaiveDate::parse_from_str(day, "%Y-%m-%d").unwrap().and_hms_opt(0, 0, 0).unwrap();
        let day1 = DateTime::<Utc>::from_naive_utc_and_offset(day, Utc).timestamp() as f64;
        (day1, *value.index(to) as f64)
    })
        .collect()
}