use chrono::{DateTime, Utc};
use finance_query_core::models::SimpleQuote;

#[derive(Debug, Clone, PartialEq)]
pub struct Price {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub percent_change: f64,
    pub pre_market_price: Option<f64>,
    pub after_hours_price: Option<f64>,
}

impl Price {
    fn parse_f64(value: Option<&str>) -> Option<f64> {
        value.and_then(|s| s.parse::<f64>().ok())
    }
}

impl From<SimpleQuote> for Price {
    fn from(quote: SimpleQuote) -> Self {
        let price = quote.price.parse::<f64>().unwrap_or(0.0);
        let change = quote.change.parse::<f64>().unwrap_or(0.0);
        let percent_change = quote
            .percent_change
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0);

        Self {
            symbol: quote.symbol,
            name: quote.name,
            price,
            change,
            percent_change,
            pre_market_price: Price::parse_f64(quote.pre_market_price.as_deref()),
            after_hours_price: Price::parse_f64(quote.after_hours_price.as_deref()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriceUpdate {
    pub prices: Vec<Price>,
    pub timestamp: DateTime<Utc>,
}

