# /quote

Fetch a simple quote for a ticker.

Usage
- Slash: `/quote ticker:<symbol>`
- Mention: `@Bot quote TICKER`

Output
- Name and symbol
- Price (with currency), change and % change
- Pre-market and after-hours prices when available

Notes
- Errors return `fetch error: …` if the finance API call fails.

