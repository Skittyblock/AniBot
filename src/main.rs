pub mod anilist;
pub mod commands;

use anilist::models::MediaType;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => match command.data.name.as_str() {
                "ping" => commands::ping::run(ctx, command).await,
                "anime" => commands::lookup::run(ctx, command, MediaType::Anime).await,
                "manga" => commands::lookup::run(ctx, command, MediaType::Manga).await,
                other_command => println!("Unknown command {}", other_command),
            },
            Interaction::Autocomplete(command) => match command.data.name.as_str() {
                "anime" => commands::lookup::autocomplete(ctx, command, MediaType::Anime).await,
                "manga" => commands::lookup::autocomplete(ctx, command, MediaType::Manga).await,
                other_command => println!("No autocomplete for {}", other_command),
            },
            other_interaction => println!("Unhandled interaction {:?}", other_interaction),
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Connected to {}#{}",
            ready.user.name, ready.user.discriminator
        );

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        _ = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                // .create_application_command(|command| {
                //     command
                //         .name("ping")
                //         .description("Check to see if the bot is running")
                // })
                .create_application_command(|command| {
                    command
                        .name("anime")
                        .description("Search anime on AniList")
                        .create_option(|option| {
                            option
                                .name("name")
                                .description("The name of the anime")
                                .kind(CommandOptionType::String)
                                .set_autocomplete(true)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("manga")
                        .description("Search manga on AniList")
                        .create_option(|option| {
                            option
                                .name("name")
                                .description("The name of the manga")
                                .kind(CommandOptionType::String)
                                .set_autocomplete(true)
                                .required(true)
                        })
                })
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
