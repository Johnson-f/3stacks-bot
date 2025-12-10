# /holders

Show holders information for a ticker.

Usage
- Slash: `/holders ticker:<symbol> type:<category> limit:[1-10]`
- Mention: `@Bot holders TICKER TYPE [LIMIT]`
- TYPE choices: `major | institutional | mutualfund | insider_transactions | insider_purchases | insider_roster`
- LIMIT defaults to 5, clamps 1–10.

Output by type
- `major`: Major holders breakdown as percentages.
- `institutional`: Top rows by shares with %out, reported date; shares formatted to M/B.
- `mutualfund`: Same format as institutional.
- `insider_transactions`: Insider buys/sells with shares (M/B) and value (M/B).
- `insider_purchases`: Aggregated recent buys/sells/net shares.
- `insider_roster`: Insider roster with direct/indirect holdings (M/B) and last transaction.

Notes
- Errors return `fetch error: …` if the finance API call fails or if no data for the selected type.

