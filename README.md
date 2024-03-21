# Currency Converter

This project is a command-line application for fetching and displaying currency exchange rates. It uses the Frankfurter API to fetch current and historical exchange rates for a wide range of currencies. The application is built with Rust and features a text-based UI for displaying charts of exchange rates over time.

## Features

- **List Available Currencies:** Users can list all available currencies with their codes.
- **Fetch Current Exchange Rate:** Fetch the current exchange rate between two specified currencies.
- **Calculate Exchange Amount:** Calculate the amount in the target currency based on the current exchange rate and a specified amount in the source currency.
- **Display Exchange Rate Chart:** Display a chart of the exchange rate between two currencies over the past month.

## Usage

To use this application, you'll need Rust installed. Once you have Rust set up, you can run the application with `cargo run`. The application supports the following flags:

- `--list`: Lists all available currencies.
- `--from <CURRENCY_CODE>`: Specifies the source currency code.
- `--to <CURRENCY_CODE>`: Specifies the target currency code.
- `--amount <AMOUNT>`: Specifies the amount in the source currency to be converted.
- `--chart`: Displays a chart of the exchange rate over the past month for the specified `from` and `to` currencies.
- `--help`: Displays help menu..

## Examples

Listing all available currencies:

```shell
cargo run --list
```

Fetching the current exchange rate from EUR to USD:

```shell
cargo run -- --from EUR --to USD
```

Calculating the amount of USD for 100 EUR:

```shell
cargo run -- --from EUR --to USD --amount 100
```

Displaying the exchange rate chart from EUR to USD over the past month:

```shell
cargo run -- --from EUR --to USD --chart
```
