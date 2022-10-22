pub mod anilist;
pub mod commands;

use anilist::models::MediaType;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::builder::CreateApplicationCommands;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::{Client, Context, EventHandler, GatewayIntents};
use std::env;

struct Handler;

fn register_cmds(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        // .create_application_command(|command| commands::ping::register(command))
        .create_application_command(|command| commands::lookup::register(command, MediaType::Anime))
        .create_application_command(|command| commands::lookup::register(command, MediaType::Manga))
}

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

        if let Ok(guild_id) = env::var("GUILD_ID") {
            let guild_id = GuildId(guild_id.parse().expect("GUILD_ID must be an integer"));
            if guild_id != 0 {
                _ = GuildId::set_application_commands(&guild_id, &ctx.http, register_cmds).await;
                return;
            }
        }
        _ = Command::set_global_application_commands(&ctx.http, register_cmds).await;
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
