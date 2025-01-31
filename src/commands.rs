use crate::Context;
use anyhow::Result;
use poise::serenity_prelude::*;

use crate::subscriptions::Subscriber;

#[poise::command(slash_command)]
pub async fn subscribe(ctx: Context<'_>, role: Option<Role>) -> Result<()> {
    ctx.data()
        .subscriptions
        .add_subscriber(Subscriber {
            channel_id: ctx.channel_id(),
            role_id: role.map(|role| role.id),
        })
        .expect("Mutex should not be poisoned");
    ctx.reply("Subscribed").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn unsubscribe(ctx: Context<'_>) -> Result<()> {
    match ctx.data().subscriptions.remove_subscriber(ctx.channel_id()) {
        Ok(Some(sub)) => {
            ctx.reply(format!("Removed {}'s subscription", sub.channel_id))
                .await?;
        }
        Ok(None) => {
            ctx.reply("Channel is not subscribed").await?;
        }
        Err(e) => return Err(e),
    };
    Ok(())
}
