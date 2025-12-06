use finance_query_core::{FetchClient, YahooAuthManager, YahooFinanceClient};
use serde_json::to_string_pretty;
use std::sync::Arc;

/// Integration test that hits the live Yahoo Finance API via finance-query-core.
///
/// This requires outbound network access. It is marked ignored by default to
/// avoid failures in offline or CI environments. Run manually with:
/// `cargo test -- --ignored fetches_live_quote`.
#[tokio::test]
#[ignore = "requires network access to Yahoo Finance"]
async fn fetches_live_quote() -> Result<(), Box<dyn std::error::Error>> {
    let fetch = Arc::new(FetchClient::new(None)?);
    let auth = Arc::new(YahooAuthManager::new(None, fetch.cookie_jar().clone()));
    let client = YahooFinanceClient::new(auth, fetch);

    let data = client.get_simple_quotes(&["AAPL"]).await?;

    // Save raw quote JSON for inspection.
    let pretty = to_string_pretty(&data)?;
    let out_path = std::path::Path::new("build-docs/stacks-bot-docs/json_output/quote_output.json");
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(out_path, &pretty)?;
    println!(
        "full quote response saved to {}:\n{}",
        out_path.display(),
        pretty
    );

    let result = data
        .get("quoteResponse")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .ok_or("no quote data returned")?;

    assert_eq!(result.get("symbol").and_then(|s| s.as_str()), Some("AAPL"));
    assert!(
        result.get("regularMarketPrice").is_some(),
        "regularMarketPrice missing"
    );

    Ok(())
}
