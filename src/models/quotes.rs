use serde::{Deserialize, Serialize};

/// Bot-facing quote model with the fields we want to surface in Discord.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceQuote {
    pub symbol: String,
    pub name: String,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub change: Option<f64>,
    pub percent_change: Option<f64>,
    pub pre_market_price: Option<f64>,
    pub after_hours_price: Option<f64>,
}