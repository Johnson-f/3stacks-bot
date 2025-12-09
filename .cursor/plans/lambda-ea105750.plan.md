<!-- ea105750-49ab-4b28-8fbf-2d5092671b2c 3a4f4ad6-c9ec-41a1-b55b-7c6634aa9983 -->
# Plan: Lambda-bot relay in Rust

- Update dependencies in `[dependencies]` of [`Lambda-bot/Cargo.toml`](Lambda-bot/Cargo.toml) to add `serenity`, `tokio`, `dotenvy`, and `tracing`.
- Implement the relay bot in [`Lambda-bot/src/main.rs`](Lambda-bot/src/main.rs):
- Load `DISCORD_TOKEN`, `SOURCE_CHANNEL_ID`, `TARGET_CHANNEL_ID` from env.
- Configure Gateway intents (`GUILD_MESSAGES`, `MESSAGE_CONTENT`).
- Event handler: ignore bots, filter source channel, apply processing (e.g., prefix with author), send to target with safe `allowed_mentions`.
- Add basic logging/tracing and error handling.
- Add an `.env` example in `Lambda-bot` documenting required vars and run instructions (`cargo run`), plus note to enable Message Content Intent in the Discord portal.

### To-dos

- [ ] Add Serenity/Tokio/dotenvy/tracing deps
- [ ] Implement relay handler and client startup
- [ ] Document env vars and run instructions