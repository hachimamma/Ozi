mod commands;

use commands::purge;
use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::env;

pub struct Data {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

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
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| Box::pin(async move {
            let guild_id = serenity::GuildId::new(1381641115618377788);
            poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
            println!("Ozi Bot is online and commands registered in guild: {}", guild_id);
            Ok(Data {})
        }))
        .build(); // <-- No .await, no .unwrap

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    client.start().await.expect("Client error");
}