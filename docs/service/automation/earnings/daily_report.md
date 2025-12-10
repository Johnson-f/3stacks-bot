# Daily Earnings Report

Scheduled automation that posts the current day’s earnings (with implied move) on weekdays at 6:00 PM ET.

What it does
- Determines target date (Fri for Sat requests, Mon for Sun requests, otherwise today).
- Fetches earnings for that date via `FinanceService::get_earnings_range`.
- Classifies each event as BMO/AMC/TBA; fetches nearest post-earnings option expiry to compute:
  - ATM call/put IVs
  - Implied move percentage (ATM call + ATM put) / spot
- Posts a text summary line per symbol: `SYMBOL [BMO/AMC/TBA] — IV C xx.x% | IM ±xx.x%` (or notes IV unavailable).
- If no events, posts a “No companies reporting” message.

Schedule and gating
- Runs every minute, posts only when `weekday ∈ Mon–Fri` and `18:00–18:04` ET.
- Skips entirely when `ENABLE_EARNINGS_PINGER=0`.
- Deduplicates per day using in-memory last-post date.

Channel selection (first valid wins)
- `EARNINGS_DAILY_CHANNEL_ID`
- `EARNINGS_CHANNEL_ID`

Key files
- Logic: `src/service/automation/earnings/daily_report.rs`
- Exports via `src/service/automation/earnings/mod.rs`

