use crate::{
    api::{self, CharacterStats, Nickname, UserStats},
    database,
    util::paginate_embeds,
    Context, Error,
};
use poise::serenity_prelude::builder::CreateEmbed;

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
    let response: Nickname = api::get_userid(username).await?;
    ctx.say(response.user.userNum.to_string()).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn link(
    ctx: Context<'_>,
    #[description = "ID of account to link to discord account"] er_id: String,
) -> Result<(), Error> {
    match database::add_data(ctx.author().id.to_string(), er_id).await {
        Ok(()) => {
            ctx.say("Uhhh uhmmm").await?;
            return Ok(());
        }
        Err(_err) => {
            ctx.say("Account already linked: test").await?;
            return Ok(());
        }
    }
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
    let account = database::query_discord_id(discord_id).await?;
    ctx.say(account.er_id).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn er(
    ctx: Context<'_>,
    #[description = "Eternal Return ID to find"] er_id: String,
) -> Result<(), Error> {
    let account = database::query_er_id(er_id).await?;
    ctx.say(account.discord_id).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn games(
    ctx: Context<'_>,
    #[description = "Account"] account: String,
    #[description = "Season (0 = Unranked)"] season: String,
) -> Result<(), Error> {
    let response: UserStats = api::get_user_stats(account, season).await?;
    let mut embeds: Vec<CreateEmbed> = vec![];
    for char in
        <Vec<CharacterStats> as Clone>::clone(&response.userStats[0].characterStats).into_iter()
    {
        let embed = CreateEmbed::new()
            .title(char.characterCode.to_string())
            .description(char.totalGames.to_string());
        embeds.push(embed);
    }
    paginate_embeds(ctx, embeds).await?;
    Ok(())
}
