# Fundamentals

## Overview

The fundamentals module is used to fetch financial data from Yahoo Finance.

## Usage

```rust
let data = fetch_fundamentals_timeseries(client, symbol, statement_type, frequency, years_back).await;
```

## Parameters

- `client`: The YahooFinanceClient to use.
- `symbol`: The symbol to fetch data for.
- `statement_type`: The statement type to fetch data for.
- `frequency`: The frequency to fetch data for.
- `years_back`: The number of years back to fetch data for.

## Returns

- `Result<Value, YahooError>`: The raw JSON response from Yahoo Finance.

## Example

```rust
let data = fetch_fundamentals_timeseries(client, "AAPL", StatementType::IncomeStatement, Frequency::Annual, 2).await;
```