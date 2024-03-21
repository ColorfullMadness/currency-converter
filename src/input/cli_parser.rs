use clap::{Parser, ArgAction};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(after_help = "Simple currency converter.")]
pub struct Cli {
    #[arg(help = "Code of base currency.")]
    pub from_currency_code: Option<String>,

    #[arg(help = "Code of target currency.")]
    pub to_currency_code: Option<String>,

    #[arg(help = "Amount of base currency to be exchanged.", requires = "from_currency_code", requires="to_currency_code")]
    pub amount: Option<f32>,

    #[arg(short = 'l', long = "list", exclusive = true, default_missing_value = "true", default_value = "false", require_equals = true, num_args = 0..=1, action = ArgAction::Set)]
    #[arg(help = "Lists all of the available currency codes and their full names.")]
    pub list: bool,

    #[arg(short = 'c', long = "chart", requires="to_currency_code", requires="from_currency_code")]
    #[arg(default_missing_value = "true", default_value = "false", require_equals = true, num_args = 0..=1, action = ArgAction::Set)]
    #[arg(help = "Displays a chart of exchange rate from the last 30 days. Requires both currency codes. ")]
    pub chart: bool,

    #[arg(short = 'i', long = "invalidate_cache")]
    #[arg(default_missing_value = "true", default_value = "false", require_equals = true, num_args = 0..=1, action = ArgAction::Set)]
    #[arg(help = "Invalidate cached list of available currencies.")]
    pub invalidate: bool,
}