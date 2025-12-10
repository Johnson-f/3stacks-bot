# SPY Options Pinger

Posts SPY option slice snapshots to a Discord channel during market hours with a chart and text summary.

What it does
- Every 15 minutes (on :00, :15, :30, :45) during 9:30–16:00 ET, fetches today’s SPY option chain (nearest expiry) via `FinanceService::get_option_slice_today`.
- Builds text summary with spot and top 5 calls above spot / puts below spot, showing LTP, bid/ask, IV, OI, Vol, ITM flag.
- Persists strike price history to Redis (if configured) and renders a line chart via QuickChart. Falls back to text-only if charting fails or Redis is unavailable.
- Deduplicates runs within the same minute to avoid double posts.

Schedule and gating
- Runs every minute but posts only on 15-minute marks during market window 9:30–16:00 ET, Mon–Fri.
- Disabled when `ENABLE_OPTIONS_PINGER=0`.

Channel selection
- `OPTIONS_CHANNEL_ID` (required)

Caching / Redis
- Optional: set `REDIS_URL` to enable persistence across restarts.
- Stores per-expiration strike series under `spy:history:{expiration}:{strike}` (JSON entries of `{t, p}`), with a 7-day TTL and 200-point cap.
- Set `REDIS_URL` or run without to use in-memory fallback (history resets on restart).

Key files
- Logic: `src/service/automation/options_data/spy_data.rs`
- Redis helpers: `src/service/caching/collections/spy_data.rs`
- Export: `src/service/automation/options_data/mod.rs`

