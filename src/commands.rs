use poise::serenity_prelude as serenity;
use rand::Rng;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use poise::serenity_prelude::Mentionable;
use poise::CreateReply;
use crate::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, crate::Data, Error>;

/// Replies with a greeting message (dont even use this)
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply::default().content(format!("Sup {}, what's good today? 👋", ctx.author().name)).ephemeral(true)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Ship two users together!"),
    rename = "ship"
)]
pub async fn ship(
    ctx: Context<'_>,
    #[description = "First user to ship (optional)"] user1: Option<serenity::User>,
    #[description = "User to ship with (optional)"] user2: Option<serenity::User>,
) -> Result<(), Error> {
    let user1 = user1.as_ref().unwrap_or(ctx.author());
    let user2 = user2.as_ref().unwrap_or(ctx.author());

    let name1 = user1.name.to_lowercase();
    let name2 = user2.name.to_lowercase();

    let score = if (name1 == "kakashi_hatake_3200" && name2 == "hachimamma")
        || (name1 == "hachimamma" && name2 == "kakashi_hatake_3200")
    {
        100
    } else {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(format!("{}{}", user1.id, user2.id).as_bytes());
        let base = hasher.finalize();
        let offset: i8 = rand::thread_rng().gen_range(-3..=3);
        ((base % 101) as i16 + offset as i16).clamp(0, 100) as u8
    };

    let emoji = match score {
        0..=30 => "💔",
        31..=70 => "❤️",
        _ => "💖",
    };

    let response = format!(
        "{emoji} Shipping {} ❤️ {} = {score}% match!",
        user1.name, user2.name
    );

    ctx.send(CreateReply::default().content(response)).await?;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("ship_log.txt")?;
    writeln!(
        file,
        "{}: {} ❤️ {} = {}%",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        user1.name,
        user2.name,
        score
    )?;

    Ok(())
}

/// Replies with pong and latency
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;
    let latency = format!("{:.2}ms", latency.as_secs_f64() * 1000.0);

    let cluster_id = 430;
    let cluster_avg_latency = 21.78;
    let shard_id = ctx.serenity_context().shard_id;
    let node_name = "Node2.carlbot-prod.local";

    let response = format!(
        "Pong! 🏓\nCluster {cluster_id}: {cluster_avg_latency:.2}ms (avg)\nShard {shard_id}: {latency}\nNode: {node_name}",
        cluster_id = cluster_id,
        cluster_avg_latency = cluster_avg_latency,
        shard_id = shard_id,
        latency = latency,
        node_name = node_name,
    );

    ctx.send(CreateReply::default().content(response)).await?;
    Ok(())
}

/// Rolls a dice between min and max
#[poise::command(slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Minimum value"] min: Option<i64>,
    #[description = "Maximum value"] max: Option<i64>,
) -> Result<(), Error> {
    let min = min.unwrap_or(1);
    let max = max.unwrap_or(6);
    if min > max {
        ctx.send(CreateReply::default().content("Minimum should be less than or equal to maximum, u baka!")).await?;
        return Ok(());
    }
    let roll = rand::thread_rng().gen_range(min..=max);
    ctx.send(CreateReply::default().content(format!("You rolled: {}", roll))).await?;
    Ok(())
}

/// Bot repeats your message (do sus things with this)
#[poise::command(slash_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Message for the bot to repeat"] text: String,
) -> Result<(), Error> {
    ctx.send(CreateReply::default().content(text)).await?;
    Ok(())
}

/// Shows info about a mentioned user
#[poise::command(slash_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "User to show info about (optional)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or(ctx.author());
    let created = user.created_at().format("%Y-%m-%d %H:%M:%S");
    let mention = user.mention();
    let discrim = user
        .discriminator
        .map(|d| d.get().to_string())
        .unwrap_or_else(|| "none".to_string());
    let response = format!(
        "**User Info:**\n\
        Name: {}\n\
        Tag: {}#{}\n\
        ID: {}\n\
        Mention: {}\n\
        Created: {}",
        user.name, user.name, discrim, user.id, mention, created
    );
    ctx.send(CreateReply::default().content(response)).await?;
    Ok(())
}

/// Shows a user's avatar
#[poise::command(slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "User to show avatar of (optional)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or(ctx.author());
    let avatar_url = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());

    ctx.send(CreateReply::default().content(avatar_url)).await?;
    Ok(())
}

/// Picks randomly from a list of options (dont ask why i added this)
#[poise::command(slash_command)]
pub async fn choose(
    ctx: Context<'_>,
    #[description = "Choices, separated by commas"] options: String,
) -> Result<(), Error> {
    let choices: Vec<_> = options.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    if choices.is_empty() {
        ctx.send(CreateReply::default().content("Please provide at least one option, baka!")).await?;
        return Ok(());
    }
    let choice = choices[rand::thread_rng().gen_range(0..choices.len())];
    ctx.send(CreateReply::default().content(format!("I choose: **{}**", choice))).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    let (guild_id, name, owner_id, member_count, created) = {
        let guild = ctx.guild().unwrap();
        (
            guild.id,
            guild.name.clone(),
            guild.owner_id,
            guild.member_count,
            guild.id.created_at().format("%Y-%m-%d %H:%M:%S").to_string()
        )
    };
    ctx.send(CreateReply::default().content(format!(
        "**Server Info:**\nName: {}\nID: {}\nOwner: <@{}>\nMembers: {}\nCreated: {}",
        name, guild_id, owner_id, member_count, created
    ))).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only,
    required_permissions = "MANAGE_MESSAGES",
    description_localized("en-US", "Bulk delete messages in the current channel.")
)]
pub async fn purge(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "Number of messages to delete (max 100)"] amount: u64,
) -> Result<(), Error> {
    use poise::serenity_prelude as serenity;

    const ALLOWED_ROLE_ID: u64 = 1390227721312927795;

    let member = match ctx.author_member().await {
        Some(member) => member,
        None => {
            ctx.send(CreateReply::default().content("Could not fetch member info. Sowwy :("))
                .await?;
            return Ok(());
        }
    };

    let has_role = member.roles.iter().any(|role| role.get() == ALLOWED_ROLE_ID);
    if !has_role {
        ctx.send(CreateReply::default().content("You do not have the required role to use this command, go away hmph!"))
            .await?;
        return Ok(());
    }

    let amount = amount.clamp(1, 100) as u8;
    let channel_id = ctx.channel_id();

    let messages = channel_id
        .messages(&ctx.http(), serenity::GetMessages::default().limit(amount))
        .await?;

    let message_ids: Vec<_> = messages.iter().map(|msg| msg.id).collect();
    channel_id.delete_messages(&ctx.http(), message_ids).await?;

    ctx.send(CreateReply::default().content(format!("Deleted {} messages! Praise me UwU", amount))).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "City name"] city: String,
) -> Result<(), Error> {
    let api_key = std::env::var("WEATHER_API").unwrap_or_else(|_| "1e4d4aa078d7c4113bcbde15470b24b4".into());
    if api_key == "1e4d4aa078d7c4113bcbde15470b24b4" {
        ctx.send(CreateReply::default().content("Weather command not configured. Please set WEATHER_API in your .env.")).await?;
        return Ok(());
    }
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);
    let resp = reqwest::get(&url).await;
    match resp {
        Ok(r) => {
            if r.status().is_success() {
                let data: serde_json::Value = r.json().await.unwrap_or_default();
                let temp = data["main"]["temp"].as_f64().unwrap_or(0.0);
                let desc = data["weather"][0]["description"].as_str().unwrap_or("unknown");
                ctx.send(CreateReply::default().content(format!("Weather in {}: {}°C, {}", city, temp, desc))).await?;
            } else {
                ctx.send(CreateReply::default().content("City not found or API error!")).await?;
            }
        }
        Err(_) => {
            ctx.send(CreateReply::default().content("Failed to fetch weather.")).await?;
        }
    }
    Ok(())
}

#[poise::command(slash_command, subcommands("spotify_status"))]
pub async fn spotify(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "status")]
pub async fn spotify_status(
    ctx: Context<'_>,
    #[description = "User to check (optional)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.send(CreateReply::default()
                .content("This command can only be used in a server.")
                .ephemeral(true)).await?;
            return Ok(());
        }
    };

    let target = user.unwrap_or_else(|| ctx.author().clone());

    let presence = ctx.serenity_context()
        .cache
        .guild(guild_id)
        .and_then(|guild| guild.presences.get(&target.id).cloned());

    let presence = match presence {
        Some(p) => p,
        None => {
            ctx.send(CreateReply::default()
                .content("This user has no presence info available.")).await?;
            return Ok(());
        }
    };

    let activity = match presence.activities.iter().find(|a| a.name == "Spotify") {
        Some(a) => a,
        None => {
            ctx.send(CreateReply::default()
                .content("No Spotify activity found for this user.")).await?;
            return Ok(());
        }
    };

    let song = activity.details.as_deref().unwrap_or("Unknown title");
    let artist = activity.state.as_deref().unwrap_or("Unknown artist");

    let (bar, percentage) = if let Some(ts) = &activity.timestamps {
        if let (Some(start_ms), Some(end_ms)) = (ts.start, ts.end) {
            let now_ms = chrono::Utc::now().timestamp_millis() as u64;
            let progress = (now_ms.saturating_sub(start_ms)) as f64;
            let total = (end_ms - start_ms) as f64;
            let pct = (progress / total).clamp(0.0, 1.0);
            let pct_display = (pct * 100.0).round() as u8;
            let filled = (pct * 20.0).round() as usize;
            let bar_str = format!(
                "[{}{}] {}%",
                "■".repeat(filled),
                "—".repeat(20 - filled),
                pct_display
            );
            (bar_str, pct_display)
        } else {
            ("[waiting…]".into(), 0)
        }
    } else {
        ("[No timestamp available]".into(), 0)
    };

    // Build reply message
    let reply = format!(
        "**{}**\n{}\n{}\n",
        song, artist, bar
    );

    ctx.send(CreateReply::default().content(reply)).await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    aliases("oziban", "ozi_ban"),
    rename = "ban",
    category = "Dumb"
)]
pub async fn ozi_ban(
    ctx: Context<'_>,
    #[description = "User to fake ban"] user: serenity::User,
    #[description = "Reason (optional)"] #[rest] _reason: Option<String>,
) -> Result<(), Error> {
    let response = format!("🔨 Banned `{}` indefinitely", user.name);
    ctx.say(response).await?;
    Ok(())
}
