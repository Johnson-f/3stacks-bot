use finance_query_core::YahooError;
use serde_json::Value;

use super::FinanceServiceError;

/// Lightweight helpers around quote responses.
pub fn extract_simple_quote(data: &Value) -> Option<crate::models::PriceQuote> {
    let result = data
        .get("quoteResponse")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())?;

    Some(crate::models::PriceQuote {
        symbol: result.get("symbol")?.as_str()?.to_string(),
        name: result
            .get("longName")
            .or_else(|| result.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or_default()
            .to_string(),
        price: result.get("regularMarketPrice").and_then(|v| v.as_f64()),
        currency: result
            .get("currency")
            .or_else(|| result.get("financialCurrency"))
            .and_then(|c| c.as_str())
            .map(|s| s.to_string()),
        change: result
            .get("regularMarketChange")
            .and_then(|v| v.as_f64()),
        percent_change: result
            .get("regularMarketChangePercent")
            .and_then(|v| v.as_f64()),
        pre_market_price: result.get("preMarketPrice").and_then(|v| v.as_f64()),
        after_hours_price: result.get("postMarketPrice").and_then(|v| v.as_f64()),
    })
}

pub fn map_error(err: YahooError) -> FinanceServiceError {
    FinanceServiceError::Yahoo(err)
}

