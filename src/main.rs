use std::env;

use anyhow::Result;
use dotenv::dotenv;
use serenity::all::{
    ApplicationId, Command, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, GatewayIntents, Interaction,
};
use serenity::{async_trait, model::gateway::Ready, prelude::*, Client};
use tracing::info;

struct Handler {
    app_id: ApplicationId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        // Register a simple global slash command (`/ping`). Extend as needed.
        let _ = Command::create_global_command(&ctx.http, ping_command()).await;
        info!("{} is connected and commands registered (app_id {}).", ready.user.name, self.app_id);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Pong!",
                _ => "Command not implemented.",
            };

            let _ = command
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(content),
                    ),
                )
                .await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let app_id_raw: u64 = env::var("APPLICATION_ID")?.parse()?;
    let app_id: ApplicationId = app_id_raw.into();

    // Empty intents - slash commands don't require any gateway intents
    let intents = GatewayIntents::empty();

    let mut client = Client::builder(token, intents)
        .application_id(app_id)
        .event_handler(Handler { app_id })
        .await?;

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why}");
    }

    Ok(())
}

fn ping_command() -> CreateCommand {
    CreateCommand::new("ping").description("Simple ping command")
}