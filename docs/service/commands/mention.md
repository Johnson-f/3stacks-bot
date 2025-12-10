# Mention commands

Text-based helper that responds to `@Bot ...` messages with the same handlers used by slash commands.

Supported patterns
- `quote TICKER`
- `holders TICKER TYPE [LIMIT]`
- `news TICKER [LIMIT]`
- `income|balance|cashflow TICKER METRIC FREQ [YEAR] [QUARTER]`
- `earnings weekly|daily|reports`

Outputs
- Mirrors the respective slash command responses (text; earnings weekly may include an image attachment).

Errors
- Returns a help text if the command is missing/unknown; passes through handler errors (e.g., fetch errors, bad args).
