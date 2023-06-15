use crate::bot::types::*;

// testing command, should always work
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    // constructs result
    let response = "Pong!";
    ctx.say(response).await?;
    Ok(()) //makes result OK
}


// Vote
#[poise::command(slash_command)]
pub async fn vote(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}




// show current stats

#[poise::command(slash_command)]
pub async fn current_votes(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

// run script (no escaping???)

#[poise::command(slash_command)]
pub async fn run_script(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
