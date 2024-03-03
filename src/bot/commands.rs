use crate::{api, database, Context, Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn test(
    ctx: Context<'_>,
    #[description = "Username"] username: String,
) -> Result<(), Error> {
    let response = api::get_userid(username).await?;
    ctx.say(response.to_string()).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn link(
    ctx: Context<'_>,
    #[description = "ID of account to link to discord account"] er_id: String,
) -> Result<(), Error> {
    database::add_data(ctx.author().id.to_string(), er_id).await?;
    ctx.say("uhhh uhmmmm").await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("discord", "er"),
    subcommand_required
)]
pub async fn query(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn discord(
    ctx: Context<'_>,
    #[description = "Discord ID to find"] discord_id: String,
) -> Result<(), Error> {
    let accounts = database::query_discord_id(discord_id).await?;
    ctx.say("wuh").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn er(
    ctx: Context<'_>,
    #[description = "Eternal Return ID to find"] er_id: String,
) -> Result<(), Error> {
    let accounts = database::query_er_id(er_id).await?;
    ctx.say("uh wuh").await?;
    Ok(())
}
