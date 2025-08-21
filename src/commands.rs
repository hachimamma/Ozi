use poise::serenity_prelude as serenity;
use rand::Rng;
use chrono::Local;
use ::serenity::all::{CreateEmbedAuthor, CreateEmbedFooter};
use std::fs::OpenOptions;
use std::io::Write;
use poise::serenity_prelude::Mentionable;
use poise::CreateReply;
use poise::serenity_prelude::{CreateEmbed, Colour};
use crate::helpers::{is_mod, load_conf, save_conf};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, crate::Data, Error>;

/// Greetings
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let mut embed = CreateEmbed::new()
        .title("Greetings!")
        .description(format!("H-Hi... don't get the wrong idea now, {}! :3", ctx.author().name))
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));
    
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Ship two users together XD
#[poise::command(
    slash_command,
    description_localized("en-US", "Ship two users together!"),
    rename = "ship"
)]
pub async fn ship(
    ctx: Context<'_>,
    #[description = "First user to ship (optional) UwU"] user1: Option<serenity::User>,
    #[description = "User to ship with (optional) UwU"] user2: Option<serenity::User>,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let user1 = user1.as_ref().unwrap_or(ctx.author());
    let user2 = user2.as_ref().unwrap_or(ctx.author());

    let name1 = user1.name.to_lowercase();
    let name2 = user2.name.to_lowercase();

    //dont question this, its personal
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

    let resp = format!(
        "{emoji} Shipping {} ❤️ {} = {score}% match! UwU",
        user1.name, user2.name
    );

    let mut embed = CreateEmbed::new()
        .title("Shipping Results")
        .description(resp)
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;

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

/// Ping me and get my latency
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let latency = ctx.ping().await;
    let latency = format!("{:.2}ms", latency.as_secs_f64() * 1000.0);

    let cluster_id = 430;
    let cls_avglat = 21.78;
    let shard_id = ctx.serenity_context().shard_id;
    let nn = "Node2.carlbot-prod.local";

    let resp = format!(
        "Pong 🏓 UwU\nCluster {cluster_id}: {cls_avglat:.2}ms (avg)\nShard {shard_id}: {latency}\nNode: {nn}",
    );

    let mut embed = CreateEmbed::new()
        .title("Ping Results")
        .description(resp)
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Find a random number between two numbers
#[poise::command(slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Minimum value"] min: Option<i64>,
    #[description = "Maximum value"] max: Option<i64>,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let min = min.unwrap_or(1);
    let max = max.unwrap_or(6);
    if min > max {
        let mut embed = CreateEmbed::new()
            .title("Error")
            .description("Minimum should be less than or equal to maximum, u baka!")
            .color(Colour::from_rgb(255, 0, 0));
        
        embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
        embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

        ctx.send(CreateReply::default().embed(embed)).await?;
        return Ok(());
    }
    
    let roll = rand::thread_rng().gen_range(min..=max);
    
    let mut embed = CreateEmbed::new()
        .title("Dice Roll")
        .description(format!("You rolled: {} UwU", roll))
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Get me to say anything u want
#[poise::command(slash_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "What u want me to say huh :3"] text: String,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let mut embed = CreateEmbed::new()
        .title("Message Repeat")
        .description(text)
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Show an user's info
#[poise::command(slash_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "User to show info about (optional) UwU"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let user = user.as_ref().unwrap_or(ctx.author());
    let created = user.created_at().format("%Y-%m-%d %H:%M:%S");
    let mention = user.mention();
    let discrim = user
        .discriminator
        .map(|d| d.get().to_string())
        .unwrap_or_else(|| "none".to_string());
    let resp = format!(
        "**User Info:**\n\
        Name: {}\n\
        Tag: {}#{}\n\
        ID: {}\n\
        Mention: {}\n\
        Created: {}",
        user.name, user.name, discrim, user.id, mention, created
    );
    
    let mut embed = CreateEmbed::new()
        .title("User Information")
        .description(resp)
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&user.name).icon_url(user.avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Show an user's avatar
#[poise::command(slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "User to show avatar of (optional) UwU"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let user = user.as_ref().unwrap_or(ctx.author());
    let avatar_url = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());

    let mut embed = CreateEmbed::new()
        .title("User Avatar")
        .description(format!("Avatar of {} Praise me UwU", user.name))
        .image(avatar_url.clone())
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&user.name).icon_url(user.avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Choose any random option from a given list
#[poise::command(slash_command)]
pub async fn choose(
    ctx: Context<'_>,
    #[description = "Choices, separated by commas"] options: String,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let choices: Vec<_> = options.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    if choices.is_empty() {
        let mut embed = CreateEmbed::new()
            .title("Error")
            .description("Provide at least one option, B-baka!")
            .color(Colour::from_rgb(255, 0, 0));
        
        embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
        embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

        ctx.send(CreateReply::default().embed(embed)).await?;
        return Ok(());
    }
    
    let choice = choices[rand::thread_rng().gen_range(0..choices.len())];
    
    let mut embed = CreateEmbed::new()
        .title("Choice Made")
        .description(format!("I choose: **{}** UwU", choice))
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Server insights by me :3
#[poise::command(slash_command, guild_only)]
pub async fn serverinfo(ctx: Context<'_>) -> Result<(), Error> {
    use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, CreateEmbedAuthor, Colour};
    
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let (guild_id, name, owner_id, member_count, created, icon_url) = {
        let guild = ctx.guild().unwrap();
        (
            guild.id,
            guild.name.clone(),
            guild.owner_id,
            guild.member_count,
            guild.id.created_at().format("%Y-%m-%d %H:%M:%S").to_string(),
            guild.icon_url().unwrap_or_default()
        )
    };
    
    let resp = format!(
        "**Server Info:**\nName: {}\nID: {}\nOwner: <@{}>\nMembers: {}\nCreated: {}",
        name, guild_id, owner_id, member_count, created
    );
    
    let mut embed = CreateEmbed::new()
        .title("Server Information")
        .description(resp)
        .color(Colour::from_rgb(255, 105, 180))
        .thumbnail(icon_url);
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));
    
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Purge messages in a channel :3
#[poise::command(
    slash_command,
    guild_only,
    description_localized("en-US", "bulk delete messages in the current channel")
)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "Number of messages to delete (max 100)"] amt: u64,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    use poise::serenity_prelude as serenity;

    let guild_id = ctx.guild_id().expect("This command is guild only");
    let is_moderator = is_mod(ctx.serenity_context(), guild_id, ctx.author().id).await?;
    
    if !is_moderator {
        let mut embed = CreateEmbed::new()
            .title("Error")
            .description("You dont have perms to use this command, B-baka! xp")
            .color(Colour::from_rgb(255, 0, 0));
        
        embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
        embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

        ctx.send(CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let amt = amt.clamp(1, 100) as u8;
    let channel_id = ctx.channel_id();

    let messages = channel_id
        .messages(&ctx.http(), serenity::GetMessages::default().limit(amt))
        .await?;

    let message_ids: Vec<_> = messages.iter().map(|msg| msg.id).collect();
    channel_id.delete_messages(&ctx.http(), message_ids).await?;

    let mut embed = CreateEmbed::new()
        .title("purge complete")
        .description(format!("Deleted {} messages! Praise me UwU", amt))
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Get information on weather of a city :3
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "City name"] city: String,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let api_key = match std::env::var("WEATHER_API") {
        Ok(key) => key,
        Err(_) => {
            let mut embed = CreateEmbed::new()
                .title("Error")
                .description("Weather command not configured. Please set WEATHER_API in your .env")
                .color(Colour::from_rgb(255, 0, 0));
            
            embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
            embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

            ctx.send(CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };

    let enc_cy = urlencoding::encode(&city);
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        api_key, enc_cy
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
            
            let mut embed = CreateEmbed::new()
                .title("Weather Information")
                .description(msg)
                .color(Colour::from_rgb(255, 105, 180));
            
            embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
            embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

            ctx.send(CreateReply::default().embed(embed)).await?;
        }
        Ok(resp) => {
            let err_text = resp.text().await.unwrap_or_else(|_| "No error details.".into());
            let error_msg = format!("City not found or API error UwU\n```{}```", err_text);
            
            let mut embed = CreateEmbed::new()
                .title("Error")
                .description(error_msg)
                .color(Colour::from_rgb(255, 0, 0));
            
            embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
            embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

            ctx.send(CreateReply::default().embed(embed)).await?;
        }
        Err(_) => {
            let mut embed = CreateEmbed::new()
                .title("Error")
                .description("Failed to fetch weather.")
                .color(Colour::from_rgb(255, 0, 0));
            
            embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
            embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

            ctx.send(CreateReply::default().embed(embed)).await?;
        }
    }
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
    let resp = format!("🔨 Banned `{}` indefinitely", user.name);
    ctx.say(resp).await?;
    Ok(())
}

/// Setup the tsun tsun messages in a channel :3
#[poise::command(
    slash_command,
    guild_only,
    description_localized("en-US", "Set the channel for tsundere messages")
)]
pub async fn setup_tsundere(
    ctx: Context<'_>,
    #[description = "Channel for tsundere messages"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let author_id = ctx.serenity_context().cache.current_user().id;
    let author = author_id.to_user(&ctx.http()).await?;
    
    let guild_id = ctx.guild_id().expect("This command is guild only");
    let is_moderator = is_mod(ctx.serenity_context(), guild_id, ctx.author().id).await?;
    
    if !is_moderator {
        let mut embed = CreateEmbed::new()
            .title("Error")
            .description("You do not have permission to use this command! B-baka!")
            .color(Colour::from_rgb(255, 0, 0));
        
        embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
        embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

        ctx.send(CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let mut config = match load_conf() {
        Ok(config) => config,
        Err(e) => {
            let mut embed = CreateEmbed::new()
                .title("Error")
                .description(format!("Failed to load config: {}", e))
                .color(Colour::from_rgb(255, 0, 0));
            
            embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
            embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

            ctx.send(CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };
    
    config.tsun_chid = Some(channel.id.get());
    
    if let Err(e) = save_conf(&config) {
        let mut embed = CreateEmbed::new()
            .title("Error")
            .description(format!("Failed to save config: {}", e))
            .color(Colour::from_rgb(255, 0, 0));
        
        embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
        embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

        ctx.send(CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let success_msg = format!(
        "The messages will be sent to {}! It's not like I wanted to send messages there or anything hmph!",
        channel.mention()
    );
    
    let mut embed = CreateEmbed::new()
        .title("Tsundere Channel Set")
        .description(success_msg)
        .color(Colour::from_rgb(255, 105, 180));
    
    embed = embed.footer(CreateEmbedFooter::new("Ozi").icon_url(author.avatar_url().unwrap_or_default()));
    embed = embed.author(CreateEmbedAuthor::new(&ctx.author().name).icon_url(ctx.author().avatar_url().unwrap_or_default()));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}