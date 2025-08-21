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
use commands::{purge, ozi_ban};

#[derive(Clone)]
pub struct Data {
    tsundere_messages: Vec<String>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Serialize, Deserialize)]
struct TsundereConfig {
    messages: Vec<String>,
}

fn load_tsundere_messages() -> Result<Vec<String>, Error> {
    let file = File::open("tsu.json")?;
    let reader = BufReader::new(file);
    let config: TsundereConfig = serde_json::from_reader(reader)?;
    Ok(config.messages)
}

async fn start_tsundere_timer(ctx: serenity::Context, data: Arc<Data>) {
    let http = ctx.http.clone();
    
    let startup_channel_id = serenity::ChannelId::new(1263067254803796030);
    if !data.tsundere_messages.is_empty() {
        let random_index = {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..data.tsundere_messages.len())
        };
        
        let random_message = &data.tsundere_messages[random_index];
        
        if let Err(e) = startup_channel_id.say(&http, random_message).await {
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
            
            if !data.tsundere_messages.is_empty() {
                let random_index = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0..data.tsundere_messages.len())
                };
                
                let random_message = &data.tsundere_messages[random_index];
                
                let channel_id = serenity::ChannelId::new(1263067254803796030);
                
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

    let tsundere_messages = match load_tsundere_messages() {
        Ok(messages) => messages,
        Err(_) => {
            println!("tsu.json not found, creating default file...");
            load_tsundere_messages()?
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
                commands::setup_tsundere(),
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
            let tsundere_messages_clone = tsundere_messages.clone();
            Box::pin(async move {
                let guild_id = serenity::GuildId::new(1263067254153805905);
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                println!("Ozi Bot is online and commands registered in guild: {}", guild_id);
                
                let data = Data {
                    tsundere_messages: tsundere_messages_clone,
                };
                
                start_tsundere_timer(ctx.clone(), Arc::new(data.clone())).await;
                
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