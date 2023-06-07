use crate::bot::types::*;

// testing command, should always work
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    // constructs result
    let response = "Pong!";
    ctx.say(response).await?;
    Ok(()) //makes result OK
}

#[poise::command(slash_command)]
pub async fn vote(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
