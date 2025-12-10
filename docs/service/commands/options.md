# /options (mention helper)

There is no slash command; options data is available via mention helper:

Usage
- Mention: `@Bot options` — currently used for SPY options pinger output (automation).

Notes
- Real-time SPY options content is primarily delivered by the automation (`service/automation/options_data/spy_data.rs`). If you need an interactive slash command, add one in `src/service/command` and wire it similarly to existing commands.

