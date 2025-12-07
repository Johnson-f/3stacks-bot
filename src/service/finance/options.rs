use std::cmp::Ordering;

use chrono::{NaiveDate, Utc};
use finance_query_core::OptionContract;

use super::{FinanceService, FinanceServiceError};

#[derive(Debug, Clone)]
pub struct OptionSlice {
    pub symbol: String,
    pub expiration: String,
    pub spot: f64,
    pub calls: Vec<OptionContract>,
    pub puts: Vec<OptionContract>,
}

impl FinanceService {
    /// Fetch available option expirations as NaiveDate values.
    pub async fn get_option_expirations(
        &self,
        symbol: &str,
    ) -> Result<Vec<NaiveDate>, FinanceServiceError> {
        let expirations = self.client.get_option_expirations(symbol).await?;
        let parsed: Vec<NaiveDate> = expirations
            .expirations
            .iter()
            .filter_map(|e| NaiveDate::parse_from_str(e, "%Y-%m-%d").ok())
            .collect();

        if parsed.is_empty() {
            return Err(FinanceServiceError::NotFound(format!(
                "no option expirations for symbol {symbol}"
            )));
        }

        Ok(parsed)
    }

    /// Fetch an option chain slice for a specific expiration and strikes around spot.
    pub async fn get_option_slice(
        &self,
        symbol: &str,
        expiration: NaiveDate,
        strikes_each_side: usize,
    ) -> Result<OptionSlice, FinanceServiceError> {
        let expiration_str = expiration.format("%Y-%m-%d").to_string();
        let chain = self
            .client
            .get_option_chain(symbol, Some(expiration_str.as_str()))
            .await?;

        let spot = chain
            .underlying_price
            .ok_or_else(|| FinanceServiceError::NotFound("no underlying price".into()))?;

        let mut calls: Vec<OptionContract> = chain
            .calls
            .into_iter()
            .filter(|c| c.strike >= spot)
            .collect();
        calls.sort_by(|a, b| float_cmp(a.strike, b.strike));
        let calls = calls.into_iter().take(strikes_each_side).collect();

        let mut puts: Vec<OptionContract> = chain
            .puts
            .into_iter()
            .filter(|p| p.strike <= spot)
            .collect();
        puts.sort_by(|a, b| float_cmp(b.strike, a.strike)); // descending
        let puts = puts.into_iter().take(strikes_each_side).collect();

        Ok(OptionSlice {
            symbol: symbol.to_uppercase(),
            expiration: expiration_str,
            spot,
            calls,
            puts,
        })
    }

    /// Fetch today’s expiration option chain and slice around spot.
    pub async fn get_option_slice_today(
        &self,
        symbol: &str,
        strikes_each_side: usize,
    ) -> Result<OptionSlice, FinanceServiceError> {
        let expirations = self.client.get_option_expirations(symbol).await?;
        let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();

        let expiration = expirations
            .expirations
            .iter()
            .find(|e| *e == &today)
            .cloned()
            .ok_or_else(|| {
                FinanceServiceError::NotFound(format!(
                    "no expiration for today ({today}) for symbol {symbol}"
                ))
            })?;

        let chain = self
            .client
            .get_option_chain(symbol, Some(expiration.as_str()))
            .await?;

        let spot = chain
            .underlying_price
            .ok_or_else(|| FinanceServiceError::NotFound("no underlying price".into()))?;

        let mut calls: Vec<OptionContract> = chain
            .calls
            .into_iter()
            .filter(|c| c.strike >= spot)
            .collect();
        calls.sort_by(|a, b| float_cmp(a.strike, b.strike));
        let calls = calls.into_iter().take(strikes_each_side).collect();

        let mut puts: Vec<OptionContract> = chain
            .puts
            .into_iter()
            .filter(|p| p.strike <= spot)
            .collect();
        puts.sort_by(|a, b| float_cmp(b.strike, a.strike)); // descending
        let puts = puts.into_iter().take(strikes_each_side).collect();

        Ok(OptionSlice {
            symbol: symbol.to_uppercase(),
            expiration,
            spot,
            calls,
            puts,
        })
    }
}

fn float_cmp(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}
