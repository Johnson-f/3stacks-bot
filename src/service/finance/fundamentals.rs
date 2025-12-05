use chrono::{Duration, Utc};
use finance_query_core::{utils::get_statement_fields, YahooFinanceClient, YahooError};
use serde_json::Value;

use crate::models::{Frequency, StatementType};

/// Fetch fundamentals timeseries data for a symbol using finance-query-core.
///
/// This builds the correct `type` list from StatementType/Frequency and queries
/// Yahoo Finance fundamentals-timeseries over a configurable lookback.
pub async fn fetch_fundamentals_timeseries(
    client: &YahooFinanceClient,
    symbol: &str,
    statement_type: StatementType,
    frequency: Frequency,
    years_back: i64,
) -> Result<Value, YahooError> {
    let now = Utc::now().timestamp();
    let start = now - Duration::days(365 * years_back).num_seconds();

    let fields = get_statement_fields(statement_type.as_str(), frequency.as_str());
    let refs: Vec<&str> = fields.iter().map(String::as_str).collect();

    client
        .get_fundamentals_timeseries(symbol, start, now, &refs)
        .await
}