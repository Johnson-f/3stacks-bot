# /weekly-earnings, /daily-earnings, /er-reports

Slash commands that mirror the earnings automations.

Commands
- `/weekly-earnings`: Weekly calendar (Mon–Fri range based on current week; Sunday uses next week). Returns an image when rendering succeeds, else text fallback (may truncate if long).
- `/daily-earnings`: Posts today’s earnings with IV/IM summary to the channel that invoked the command.
- `/er-reports`: Posts post-earnings (BMO/AMC) results to the channel that invoked the command; before 4pm ET shows BMO, after 6pm ET shows AMC, between 4–6pm ET sends a waiting message.

Errors
- Surface finance fetch or timeout errors as text responses.

