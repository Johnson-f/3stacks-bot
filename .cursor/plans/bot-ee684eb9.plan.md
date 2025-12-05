<!-- ee684eb9-7425-4399-a647-82b484cc17d1 ad3da24b-fd51-4297-8c83-c541272d1f28 -->
# Plan: Add models and finance service wrapper

1) Define bot-facing models

- Add quote and financial summary structs (e.g., `PriceQuote`, `FinancialSummary`) under [`src/models`](src/models) to match Discord responses.
- Include minimal fields: symbol, name, price, change/percent, currency; financials: revenue, eps, pe, market_cap, optionally range for statements.
- Update [`src/models/mod.rs`](src/models/mod.rs) to expose these types.

2) Implement finance client wrapper

- In [`src/service/finance`](src/service/finance), create a wrapper that builds `FetchClient`, `YahooAuthManager`, and `YahooFinanceClient` once.
- Expose async fns: `get_price(symbol: &str) -> PriceQuote` (uses `get_simple_quotes`); `get_financials(symbol: &str) -> FinancialSummary` (uses `get_fundamentals_timeseries` + constants).
- Handle `YahooError` mapping into a local error type (re-exported).

3) Wire module exports and ready for commands

- Update [`src/service/finance/mod.rs`](src/service/finance/mod.rs) to re-export the client and functions.
- Ensure types are ready to be consumed by command handlers (to be implemented later).

### To-dos

- [ ] Create bot-facing quote/financial models and expose mod
- [ ] Implement finance client wrapper using finance-query-core
- [ ] Wire mod exports for service/finance and models