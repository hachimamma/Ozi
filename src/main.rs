use poise::serenity_prelude::{self as serenity, FullEvent};
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use rand::Rng;
use serde::{Deserialize, Serialize};

mod commands;
mod helpers;
use commands::{purge, ozi_ban, setup_tsundere};
use helpers::{load_conf};

#[derive(Clone)]
pub struct Data {
    tsun_msgs: Vec<String>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Serialize, Deserialize)]
struct TsundereConfig {
    messages: Vec<String>,
}

fn load_tsunmsgs() -> Result<Vec<String>, Error> {
    let file = File::open("tsu.json")?;
    let reader = BufReader::new(file);
    let config: TsundereConfig = serde_json::from_reader(reader)?;
    Ok(config.messages)
}

async fn start_tsunt(ctx: serenity::Context, data: Arc<Data>) {
    let http = ctx.http.clone();
    
    let config = match load_conf() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {} UwU", e);
            return;
        }
    };
    
    let channel_id = match config.tsun_chid {
        Some(id) => serenity::ChannelId::new(id),
        _none => {
            println!("No tsundere channel configured. Tsundere messages disabled. Use /setup_tsundere to enable");
            return;
        }
    };
    
    //to test functionality
    if !data.tsun_msgs.is_empty() {
        let random_index = {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..data.tsun_msgs.len())
        };
        
        let random_message = &data.tsun_msgs[random_index];
        
        if let Err(e) = channel_id.say(&http, random_message).await {
            eprintln!("Error sending startup message: {}", e);
        }
    }
    
    tokio::spawn(async move {
        loop {
            let delay_minutes = {
                let mut rng = rand::thread_rng();
                rng.gen_range(10..=30)
            };
            let delay = Duration::from_secs(delay_minutes * 60);
            
            sleep(delay).await;
            
            if !data.tsun_msgs.is_empty() {
                let random_index = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0..data.tsun_msgs.len())
                };
                
                let random_message = &data.tsun_msgs[random_index];
                
                if let Err(e) = channel_id.say(&http, random_message).await {
                    eprintln!("Error sending tsundere message: {}", e);
                }
            }
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let tsun_msgs = match load_tsunmsgs() {
        Ok(messages) => messages,
        Err(_) => {
            println!("tsu.json not found, creating default file...");
            let default_messages = TsundereConfig {
                messages: vec![
                    "B-baka! It's not like I wanted to talk to you or anything!".to_string(),
                    "Hmph! Don't get the wrong idea, I just happened to be here!".to_string(),
                    "I-I'm not doing this because I like you or anything!".to_string(),
                    "You're such an idiot... but I guess you're my idiot.".to_string(),
                    "Don't touch me! ...Well, maybe just a little.".to_string(),
                    "I'm not blushing! It's just... hot in here!".to_string(),
                    "You really are the worst... but I can't help liking you.".to_string(),
                    "It's not like I was waiting for you! I just had nothing better to do!".to_string(),
                    "Why are you so annoying? ...But don't go away.".to_string(),
                    "I could do better than you anytime! ...But I don't want to.".to_string(),
                ],
            };
            let file = File::create("tsu.json")?;
            serde_json::to_writer_pretty(file, &default_messages)?;
            println!("Created default tsu.json file");
            load_tsunmsgs()?
        }
    };

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_PRESENCES
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::hello(),
                commands::roll(),
                commands::say(),
                commands::userinfo(),
                commands::avatar(),
                commands::choose(),
                commands::serverinfo(),
                commands::weather(),
                commands::ship(),
                purge(),
                ozi_ban(),
                commands::ping(),
                setup_tsundere(),
            ],
            on_error: |err| Box::pin(async move {
                let _ = poise::builtins::on_error(err).await;
            }),
            event_handler: |ctx, event, _framework, _data| Box::pin(async move {
                if let FullEvent::Message { new_message } = event {
                    if new_message.author.bot {
                        return Ok(());
                    }

                    let content = new_message.content.to_lowercase();

                    let tags: HashMap<&str, &str> = HashMap::from([
                        ("sybau", "sybau ts pmo 💔🥀"),
                        ("kakashi", "kakashi hatake chad fr 🗿"),
                        ("hachimamma", "hachimamma chad fr 🗿"),
                        ("bro can u wel", "welcome, nya! 😽")
                    ]);

                    for (trigger, response) in tags.iter() {
                        if content.contains(trigger) {
                            if let Err(e) = new_message.channel_id.say(&ctx.http, *response).await {
                                eprintln!("Error sending tag: {}", e);
                            }
                            break;
                        }
                    }
                }
                Ok(())
            }),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            let tsun_msgs_clone = tsun_msgs.clone();
            Box::pin(async move {
                let guild_id = serenity::GuildId::new(1263067254153805905);
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                println!("Ozi Bot is online and commands registered in guild: {} UwU", guild_id);
                
                let data = Data {
                    tsun_msgs: tsun_msgs_clone,
                };
                
                start_tsunt(ctx.clone(), Arc::new(data.clone())).await;
                
                Ok(data)
            })
        })
        .build();

    let mut client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .await?;

    client.start().await?;
    Ok(())
}