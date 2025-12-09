# Lambda-bot relay (Rust + serenity)

## Setup
1. Copy `env.example` to `.env` and fill in `DISCORD_TOKEN`, `SOURCE_CHANNEL_ID`, `TARGET_CHANNEL_ID`.
2. In the Discord Developer Portal (Bot tab), enable **MESSAGE CONTENT INTENT** so the bot can read message bodies.
3. Invite the bot to both servers with permission to read in the source channel and send in the target channel.

## Run
```
cargo run
```

The bot listens on `SOURCE_CHANNEL_ID`, processes each user message (currently prefixes with the author), and relays it to `TARGET_CHANNEL_ID` with mentions disabled.

