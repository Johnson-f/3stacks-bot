# /news

Fetch latest headlines for a ticker.

Usage
- Slash: `/news ticker:<symbol> limit:<1-10>`
- Mention: `@Bot news TICKER [LIMIT]` (default 1)

Output
- Title with link, source, published time (UTC)
- Limits to the requested count (1–10)

Notes
- Errors return `fetch error: …` if the finance API call fails.

