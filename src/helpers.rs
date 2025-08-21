use poise::serenity_prelude as serenity;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

pub async fn is_mod(ctx: &serenity::Context, guild_id: serenity::GuildId, user_id: serenity::UserId) -> Result<bool, Error> {
    let member = guild_id.member(ctx, user_id).await?;
    let guild = guild_id.to_partial_guild(ctx).await?;
    
    let permissions = guild.member_permissions(&member);
    
    if permissions.manage_messages() || permissions.administrator() {
        return Ok(true);
    }
    
    Ok(false)
}

pub async fn _has_role(ctx: &serenity::Context, guild_id: serenity::GuildId, user_id: serenity::UserId, role_id: u64) -> Result<bool, Error> {
    let member = guild_id.member(ctx, user_id).await?;
    Ok(member.roles.iter().any(|role| role.get() == role_id))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub tsun_chid: Option<u64>,
}

pub fn load_conf() -> Result<BotConfig, Box<dyn std::error::Error + Send + Sync>> {
    match File::open("bot_config.json") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let config: BotConfig = serde_json::from_reader(reader)?;
            Ok(config)
        }
        Err(_) => {
            // File doesn't exist, create default config
            let default_config = BotConfig { tsun_chid: None };
            save_conf(&default_config)?;
            Ok(default_config)
        }
    }
}

pub fn save_conf(config: &BotConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file = File::create("bot_config.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, config)?;
    Ok(())
}