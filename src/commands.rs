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

/// Command to greet the user
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

/// Command to ping the bot and get latency info
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
    );

    ctx.send(CreateReply::default().content(response)).await?;
    Ok(())
}

///Find a random number between min and max
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

/// Command to repeat a message
#[poise::command(slash_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Message for the bot to repeat"] text: String,
) -> Result<(), Error> {
    ctx.send(CreateReply::default().content(text)).await?;
    Ok(())
}

/// Command to show user info
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

/// Command to show user avatar
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

/// Command to choose a random option from a list
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

/// Command to show server info
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

/// Command to purge messages in a channel
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

/// Command to get weather information for a city
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "City name"] city: String,
) -> Result<(), Error> {
    let api_key = match std::env::var("WEATHERAPI_KEY") {
        Ok(key) => key,
        Err(_) => {
            ctx.send(CreateReply::default().content(
                "Weather command not configured. Please set WEATHERAPI_KEY in your .env.",
            ))
            .await?;
            return Ok(());
        }
    };

    let encoded_city = urlencoding::encode(&city);
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        api_key, encoded_city
    );

    println!("DEBUG URL: {}", url);

    match reqwest::get(&url).await {
        Ok(resp) if resp.status().is_success() => {
            let json: serde_json::Value = resp.json().await.unwrap_or_default();
            let temp = json["current"]["temp_c"].as_f64().unwrap_or(0.0);
            let condition = json["current"]["condition"]["text"]
                .as_str()
                .unwrap_or("unknown");

            let msg = format!("Weather in **{}**: **{}°C**, {}", city, temp, condition);
            ctx.send(CreateReply::default().content(msg)).await?;
        }
        Ok(resp) => {
            let err_text = resp.text().await.unwrap_or_else(|_| "No error details.".into());
            ctx.send(CreateReply::default().content(format!(
                "City not found or API error!\n```{}```",
                err_text
            ))).await?;
        }
        Err(_) => {
            ctx.send(CreateReply::default().content("Failed to fetch weather.")).await?;
        }
    }

    Ok(())
}

/// Command to fake ban a user
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
