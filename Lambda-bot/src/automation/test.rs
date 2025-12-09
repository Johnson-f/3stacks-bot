use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateAllowedMentions, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateMessage, GuildId,
    Interaction, InteractionResponseFlags, MessageFlags,
};
use serenity::builder::{CreateCommand, GetMessages};
use serenity::http::Http;

pub const COMMAND_NAME: &str = "relay_messages";

/// Register the slash command for a guild (faster propagation).
pub async fn register_commands(http: &Http, guild_id: GuildId) -> serenity::Result<()> {
    let command = CreateCommand::new(COMMAND_NAME)
        .description("Relay recent messages from SOURCE_CHANNEL_ID to TARGET_CHANNEL_ID");

    guild_id.create_command(http, command).await.map(|_| ())
}

pub async fn handle_interaction(
    ctx: &Context,
    interaction: &Interaction,
    source_channel: ChannelId,
    target_channel: ChannelId,
) -> serenity::Result<()> {
    if let Interaction::Command(command) = interaction {
        if command.data.name == COMMAND_NAME {
            return relay_messages(ctx, command, source_channel, target_channel).await;
        }
    }
    Ok(())
}

async fn relay_messages(
    ctx: &Context,
    command: &CommandInteraction,
    source_channel: ChannelId,
    target_channel: ChannelId,
) -> serenity::Result<()> {
    let allowed = CreateAllowedMentions::new()
        .everyone(false)
        .all_users(false)
        .all_roles(false)
        .empty_users()
        .empty_roles();

    // Defer early to avoid 3s timeout and "Unknown interaction".
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().flags(InteractionResponseFlags::EPHEMERAL),
            ),
        )
        .await?;

    let messages = source_channel
        .messages(&ctx.http, GetMessages::new().limit(20))
        .await?;

    for msg in messages.iter().rev() {
        if msg.author.bot {
            continue;
        }

        if msg.content.trim().is_empty() {
            continue;
        }

        let processed = format!("[{}] {}", msg.author.name, msg.content);
        target_channel
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content(processed)
                    .allowed_mentions(allowed.clone()),
            )
            .await?;
    }

    command
        .create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new()
                .content("Relay completed.")
                .allowed_mentions(allowed)
                .flags(MessageFlags::EPHEMERAL),
        )
        .await?;

    Ok(())
}
