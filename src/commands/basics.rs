use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("i"))]
pub async fn help(
    ctx: Context<'_>,
    #[description = "help command"] command: Option<String>,
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
