use chrono::Utc;
use serenity::all::{CommandInteraction, CreateCommand};
use tokio::time::{timeout, Duration};
use tracing::{error, info, warn};

use crate::models::EarningsEvent;
use crate::service::finance::FinanceService;

pub fn register_command() -> CreateCommand {
    CreateCommand::new("earnings").description("Next 7 days of earnings for watchlist symbols")
}

pub async fn handle(
    _command: &CommandInteraction,
    finance: &FinanceService,
) -> Result<String, String> {
    info!("Starting earnings command handler");
    
    let start = Utc::now().date_naive();
    let end = start + chrono::Duration::days(7);

    info!("Fetching earnings from {} to {}", start, end);
    
    // Wrap the entire fetch in a timeout
    let events = match timeout(
        Duration::from_secs(20), // 20 second total timeout
        finance.get_earnings_range(start, end)
    )
    .await
    {
        Ok(Ok(events)) => {
            info!("Successfully fetched {} earnings events", events.len());
            events
        }
        Ok(Err(e)) => {
            error!("Failed to fetch earnings: {}", e);
            return Err(format!("Failed to fetch earnings: {}", e));
        }
        Err(_) => {
            error!("Earnings fetch timed out after 20 seconds");
            return Err("Request timed out. The earnings API is taking too long to respond. Please try again later.".to_string());
        }
    };

    if events.is_empty() {
        info!("No earnings found in the next 7 days");
        return Ok("No earnings within the next 7 days.".to_string());
    }

    info!("Formatting output for {} events", events.len());
    let output = format_output(&events);
    
    // Discord has a 2000 character limit
    if output.len() > 1900 {
        warn!("Output is {} characters, truncating to fit Discord limit", output.len());
        let truncated = format!(
            "{}\n\n⚠️ *Message truncated - showing first {} of {} events. Use filters to see more.*",
            &output[..1800],
            output.matches("📈").count().min(30),
            events.len()
        );
        Ok(truncated)
    } else {
        info!("Output is {} characters, within Discord limit", output.len());
        Ok(output)
    }
}

pub fn format_output(events: &[EarningsEvent]) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "📊 **Earnings Calendar (Next 7 Days)**\nFetched: {} | Total: {}",
        Utc::now().format("%Y-%m-%d %H:%M UTC"),
        events.len()
    ));
    lines.push(String::new()); // Empty line for spacing

    // Limit to first 50 events to avoid message being too long
    let display_events = if events.len() > 50 {
        warn!("Limiting display to first 50 of {} events", events.len());
        &events[..50]
    } else {
        events
    };

    for event in display_events {
        let date_str = event.date.format("%m/%d").to_string(); // Shorter date format
        let tod = match event.time_of_day.as_deref() {
            Some("16:00") | Some("amc") => "AMC",
            Some("09:00") | Some("bmo") => "BMO",
            Some(t) => t,
            None => "TBA",
        };
        
        let emoji = event.emoji.as_deref().unwrap_or("📈");
        let importance_indicator = match event.importance {
            Some(5) => " 🔥",
            Some(4) => " ⭐",
            _ => "",
        };
        
        // Compact format: emoji symbol date (time) importance
        let line = format!(
            "{} **{}** {} ({}){}", 
            emoji, 
            event.symbol, 
            date_str, 
            tod,
            importance_indicator
        );
        
        lines.push(line);
    }

    if events.len() > 50 {
        lines.push(String::new());
        lines.push(format!("*...and {} more*", events.len() - 50));
    }

    lines.join("\n")
}