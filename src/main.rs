mod commands;
mod subscriptions;

use chrono::{Utc, Weekday};
use poise::serenity_prelude as serenity;
use std::env::var;
use std::sync::Arc;
use subscriptions::SubscriptionStore;
use tokio_schedule::Job;

// Types used by all command functions
type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    subscriptions: Arc<SubscriptionStore>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want
    // to customize and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to start bot: {:?}", error)
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    //// Load config from environment ////
    // WAFFLECORD_SUBSCRIBERS_DIR: Directory where sled can store the list of
    //   subscribed channels
    // DISCORD_TOKEN: Token for the discord bot
    let subscriptions = Arc::new(
        SubscriptionStore::try_load(
            var("WAFFLECORD_SUBSCRIBERS_DIR")
                .expect("Missing `WAFFLECORD_SUBSCRIBERS_DIR` env var")
                .into(),
        )
        .expect("Error loading subscribers database."),
    );
    let token = var("DISCORD_TOKEN").expect(
        "Missing `DISCORD_TOKEN` env var, see README for more information.",
    );

    //// Configure poise framework ////
    let commands: Vec<poise::Command<Data, Error>> =
        vec![commands::subscribe(), commands::unsubscribe()];

    let options = poise::FrameworkOptions {
        commands,
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!(
                    "Executing command {}...",
                    ctx.command().qualified_name
                );
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        ..Default::default()
    };

    let subscriptions_clone = subscriptions.clone();
    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(
                    ctx,
                    &framework.options().commands,
                )
                .await?;
                Ok(Data {
                    subscriptions: subscriptions_clone,
                })
            })
        })
        .options(options)
        .build();

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    //// Schedule notifications ////
    let subscriptions_clone = subscriptions.clone();
    let http_clone = client.http.clone();
    let notification_task = tokio_schedule::every(1)
        .week()
        .on(Weekday::Wed)
        .at(12, 00, 00)
        .in_timezone(&Utc)
        .perform(move || {
            let subscriptions = subscriptions_clone.clone();
            let http = http_clone.clone();
            async move {
                let notification = String::from("Test Message");
                for sub in subscriptions.subscribers_iter() {
                    http.send_message(sub.channel_id, vec![], &notification)
                        .await
                        .expect("Failed to send message");
                }
            }
        });
    tokio::spawn(notification_task);

    //// Start the bot ////
    client.start().await.unwrap()
}
