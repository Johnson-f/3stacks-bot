# Weekly Earnings Poster

Scheduled automation that posts an earnings calendar image every Sunday at 5:00 PM ET.

What it does
- Pulls earnings events for the coming week (Sun–Fri) via `FinanceService::get_earnings_range`.
- Renders a calendar image with company logos (fetched from URLs) grouped by BMO/AMC.
- Falls back to text output using `service::command::earnings::format_output` if image render fails.
- Posts once per Sunday run; ignores the same day if already posted.

Schedule and gating
- Runs every minute, posts only when `weekday == Sunday` and `17:00–17:04` ET.
- Skips entirely when `ENABLE_EARNINGS_PINGER=0`.

Channel selection (first valid wins)
- `EARNINGS_WEEKLY_CHANNEL_ID`
- `EARNINGS_CHANNEL_ID`

Key files
- Logic: `src/service/automation/earnings/weekly_report.rs`
- Exports via `src/service/automation/earnings/mod.rs`

