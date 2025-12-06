use std::{env, sync::Arc};

use chrono::{Timelike, Utc};
use once_cell::sync::Lazy;
use serenity::all::Http;
use serenity::model::prelude::ChannelId;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::service::command::earnings::format_output;
use crate::service::finance::FinanceService;

static LAST_POST_DATE: Lazy<Mutex<Option<chrono::NaiveDate>>> =
    Lazy::new(|| Mutex::new(None));

/// Spawn a daily earnings poster (once per day).
pub fn spawn_earnings_poster(
    http: Arc<Http>,
    finance: Arc<FinanceService>,
) -> Option<JoinHandle<()>> {
    if env::var("ENABLE_EARNINGS_PINGER")
        .map(|v| v == "0")
        .unwrap_or(false)
    {
        info!("Earnings poster disabled via ENABLE_EARNINGS_PINGER=0");
        return None;
    }

    let channel_id = match env::var("EARNINGS_CHANNEL_ID")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
    {
        Some(id) => ChannelId::new(id),
        None => {
            info!("EARNINGS_CHANNEL_ID not set; earnings poster not started");
            return None;
        }
    };

    info!("Starting earnings poster to channel {}", channel_id);

    Some(tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1800)); // every 30 minutes
        loop {
            interval.tick().await;
            if should_post_now().await {
                if let Err(e) = post_once(&http, &finance, channel_id).await {
                    error!("earnings poster iteration failed: {e}");
                }
            }
        }
    }))
}

async fn post_once(
    http: &Http,
    finance: &FinanceService,
    channel_id: ChannelId,
) -> Result<(), String> {
    let start = chrono::Utc::now().date_naive();
    let end = start + chrono::Duration::days(7);

    let events = finance
        .get_earnings_range(start, end)
        .await
        .map_err(|e| format!("fetch error: {e}"))?;

    if events.is_empty() {
        info!("No earnings in next 7 days; skipping post");
        return Ok(());
    }

    let content = format_output(&events);
    channel_id
        .say(http, content)
        .await
        .map_err(|e| format!("failed to post earnings calendar: {e}"))?;

    Ok(())
}

async fn should_post_now() -> bool {
    let now = Utc::now();
    // Target hour: 13:00-13:29 UTC (~9:00 AM ET)
    let in_window = now.hour() == 13;
    if !in_window {
        return false;
    }

    let today = now.date_naive();
    let mut last = LAST_POST_DATE.lock().await;
    if let Some(prev) = *last {
        if prev == today {
            return false;
        }
    }
    *last = Some(today);
    true
}