pub mod bot;

use crate::bot::get_config;
use crate::bot::ping;
use crate::bot::types::*;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    println!("We're running 🐸");

    let json: bot::Config = get_config();

    let token: String = json.token;
    // try running a discord bot with the token??
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });
    framework.run().await.unwrap();
}
