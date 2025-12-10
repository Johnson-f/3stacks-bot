# After-Daily Earnings (Post-earnings Snapshots)

Scheduled automation that posts EPS/Revenue actuals twice per weekday: BMO (8:45 AM ET) and AMC (5:50 PM ET).

What it does
- Chooses session based on current time: BMO before 4:00 PM ET, AMC after 5:50 PM ET, otherwise sends a waiting message.
- Fetches earnings events for target dates (today; weekend handling: Sat shows Fri & Sun, Sun shows Fri).
- Filters events to the session (BMO or AMC) and pulls latest actuals from Yahoo `earnings` quote summary.
- Formats per symbol: `SYMBOL [BMO/AMC YYYY-MM-DD] — EPS <val|N/A> | Revenue <val|N/A>` (revenue auto-scales to M/B).
- If no matching results yet, posts a “no results detected yet” notice.

Schedule and gating
- Runs every minute; posts only in the session windows above.
- Skips entirely when `ENABLE_EARNINGS_PINGER=0`.
- Deduplicates BMO and AMC posts separately per day.

Channel selection (first valid wins)
- `EARNINGS_AFTER_CHANNEL_ID`
- `EARNINGS_CHANNEL_ID`

Key files
- Logic: `src/service/automation/earnings/after_daily_report.rs`
- Exports via `src/service/automation/earnings/mod.rs`

