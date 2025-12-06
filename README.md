# Discord-bot
This repo will contain the codes of a Discord bot for the financial markets for stacks trading server

## Turso/libsql configuration
- Required env vars:
  - `LIBSQL_URL=libsql://<db-name>-<org>.turso.io`
  - `LIBSQL_AUTH_TOKEN=<turso-auth-token>`
- Required table (create once in Turso): `CREATE TABLE watchlist_symbols (symbol TEXT PRIMARY KEY);`
- Populate symbols you want the bot to track: `INSERT INTO watchlist_symbols(symbol) VALUES ('AAPL'), ('MSFT');`

## Earnings features
- Slash command `earnings` returns the next 7 days of earnings for the watchlist symbols.
- Scheduled poster (daily ~13:00 UTC) posts to `EARNINGS_CHANNEL_ID`; enable unless `ENABLE_EARNINGS_PINGER=0`.
