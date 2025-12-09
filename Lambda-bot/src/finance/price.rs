use crate::models::price::{Price, PriceUpdate};
use finance_query_core::{
    client::error::YahooError, FetchClient, QuoteStream, YahooAuthManager, YahooFinanceClient,
};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;

pub struct PriceService {
    auth: Arc<YahooAuthManager>,
    client: Arc<YahooFinanceClient>,
}

impl PriceService {
    /// Create a new price service and refresh authentication.
    pub async fn new() -> Result<Self, YahooError> {
        let fetch_client = Arc::new(FetchClient::new(None)?);
        let cookie_jar = fetch_client.cookie_jar().clone();
        let auth = Arc::new(YahooAuthManager::new(None, cookie_jar));
        let client = Arc::new(YahooFinanceClient::new(auth.clone(), fetch_client));

        auth.refresh().await?;

        Ok(Self { auth, client })
    }

    /// Stream price updates for a list of symbols at the given interval.
    pub fn stream_prices(
        &self,
        symbols: Vec<String>,
        interval: Duration,
    ) -> impl futures_util::Stream<Item = Result<PriceUpdate, YahooError>> + Send {
        let client = self.client.clone();

        QuoteStream::create(client, symbols, interval).map(|res| {
            res.map(|update| PriceUpdate {
                prices: update.quotes.into_iter().map(Price::from).collect(),
                timestamp: update.timestamp,
            })
        })
    }
}

