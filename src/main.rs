use poise::serenity_prelude::{self as serenity, FullEvent};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

mod commands;
use commands::purge;

pub struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::hello(),
                commands::ship(),
                commands::ping(),
                commands::roll(),
                commands::say(),
                commands::userinfo(),
                commands::avatar(),
                commands::choose(),
                commands::serverinfo(),
                purge(),
                commands::weather(),
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
                        ("ozi", "ozi top G 🔝"),
                        ("adudakqua", "adu is a bitch."),
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
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| Box::pin(async move {
            let guild_id = serenity::GuildId::new(1263067254153805905);
            poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
            println!("Ozi Bot is online and commands registered in guild: {}", guild_id);
            Ok(Data {})
        }))
        .build();

    let mut client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .await?;

    client.start().await?;
    Ok(())
}
