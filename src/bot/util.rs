use poise::serenity_prelude::{
    ComponentInteractionCollector, CreateActionRow, CreateButton, CreateEmbed,
    CreateInteractionResponseMessage,
};

use crate::{Context, Error};

pub async fn paginate_embeds(ctx: Context<'_>, embeds: Vec<CreateEmbed>) -> Result<(), Error> {
    let ctx_id = ctx.id();
    let ctx_author_id = ctx.author().id;
    let prev_button_id = format!("{}_prev", ctx_id);
    let next_button_id = format!("{}_next", ctx_id);

    let reply = {
        let components = CreateActionRow::Buttons(vec![
            CreateButton::new(&prev_button_id).emoji("⬅".chars().next().unwrap()),
            CreateButton::new(&next_button_id).emoji("➡".chars().next().unwrap()),
        ]);

        poise::CreateReply::default()
            .embed(embeds[0].clone())
            .components(vec![components])
    };

    ctx.send(reply).await?;

    let mut current_page: usize = 0;

    while let Some(press) = ComponentInteractionCollector::new(ctx)
        .filter(move |press| {
            press.user.id == ctx_author_id && press.data.custom_id.starts_with(&ctx_id.to_string())
        })
        .timeout(std::time::Duration::from_secs(180))
        .await
    {
        if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(embeds.len() - 1);
        } else if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= embeds.len() {
                current_page = 0;
            }
        } else {
            continue;
        }

        press
            .create_response(
                ctx.serenity_context(),
                poise::serenity_prelude::CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new().embed(embeds[current_page].clone()),
                ),
            )
            .await?;
    }

    Ok(())
}
