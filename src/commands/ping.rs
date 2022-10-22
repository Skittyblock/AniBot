use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::*;
use std::time::Instant;

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    let start = Instant::now();

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("pinging..."))
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }

    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |message| {
            let duration = start.elapsed();
            message.content(format!("pong {}ms", duration.as_millis()))
        })
        .await
    {
        println!("Cannot edit slash command: {}", why);
    }
}
