use crate::anilist;
use crate::anilist::models::MediaType;
use serde::{self, Serialize};
use serde_json::json;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::autocomplete::AutocompleteInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::Context;

#[derive(Serialize, Debug)]
struct AutocompleteOption {
    name: String,
    value: String,
}

// truncate string to max_chars and append ellipsis if actually truncated.
fn truncate(mut s: String, max_chars: usize) -> String {
    if let Some((idx, _)) = s.char_indices().nth(max_chars) {
        s.replace_range(idx.., "...")
    }
    s
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction, media_type: MediaType) {
    let search = &command
        .data
        .options
        .get(0)
        .expect("Expected name option")
        .resolved
        .as_ref()
        .expect("Expected string");

    if let CommandDataOptionValue::String(query) = search {
        let result = if let Ok(id) = query.parse::<i32>() {
            anilist::query_media(id).await // search by id
        } else {
            anilist::query_search(query, media_type).await // search by query string
        };
        if let Ok(result) = result {
            if let Some(res) = result.data {
                let media = res.media;
                let id = media.id;
                let title: String = match media.title {
                    Some(title) => title.english.unwrap_or(title.romaji.unwrap_or_default()),
                    None => String::default(),
                };
                let description = media.description;
                let status = match media.status {
                    Some(anilist::MediaStatus::Releasing) => match media_type {
                        MediaType::Anime => "Airing",
                        MediaType::Manga => "Ongoing",
                    },
                    Some(s) => s.str(),
                    None => "",
                };
                let score = media.average_score.unwrap_or(0);
                let (image_url, color) = match media.cover_image {
                    Some(image) => {
                        // parse hex color
                        let mut color: u32 = 0;
                        let hex = image.color.unwrap_or_default();
                        if hex.chars().next().unwrap_or_default() == '#' && hex.len() == 7 {
                            let (r, g, b) = (
                                u8::from_str_radix(&hex[1..3], 16),
                                u8::from_str_radix(&hex[3..5], 16),
                                u8::from_str_radix(&hex[5..7], 16),
                            );
                            if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
                                color = ((r as u32) << 16) + ((g as u32) << 8) + (b as u32);
                            }
                        }
                        (image.extra_large, color)
                    }
                    None => (None, 0),
                };
                let banner_url = media.banner_image;
                let url = match media_type {
                    MediaType::Anime => format!("https://anilist.co/anime/{}", id),
                    MediaType::Manga => format!("https://anilist.co/manga/{}", id),
                };

                let episodes = media.episodes.unwrap_or(0);
                let chapters = media.chapters.unwrap_or(0);
                let format = media.format.unwrap_or(anilist::MediaFormat::Tv);
                let skip_episodes = (episodes == 0 && chapters == 0)
                    || format == anilist::MediaFormat::Movie
                    || format == anilist::MediaFormat::Music
                    || format == anilist::MediaFormat::OneShot;

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.embed(|e| {
                                    e.title(title);
                                    if let Some(description) = description {
                                        e.description(
                                            truncate(description, 250)
                                                .replace("<br><br>", "\n")
                                                .replace("<br>", "")
                                                .replace("<i>", "*")
                                                .replace("</i>", "*")
                                        );
                                    }
                                    e.field("Status", status, true);

                                    if !skip_episodes {
                                        match media_type {
                                            MediaType::Anime => {
                                                _ = e.field("Episodes", episodes.to_string(), true)
                                            }
                                            MediaType::Manga => {
                                                _ = e.field("Chapters", chapters.to_string(), true)
                                            }
                                        }
                                    }
                                    e.field(
                                        "Score",
                                        if score == 0 {
                                            String::from("N/A")
                                        } else {
                                            format!("{}%", score)
                                        },
                                        true,
                                    );
                                    // e.field("View on AniList", url, false);
                                    if let Some(image_url) = image_url {
                                        e.thumbnail(image_url);
                                    }
                                    if let Some(banner_url) = banner_url {
                                        e.image(banner_url);
                                    }
                                    e.color(color);
                                    e.url(url);
                                    e
                                })
                            })
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
                return;
            }
        }
    }

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| m.content("No anime found"))
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}

// fetch titles and ids for a query string while typing
pub async fn autocomplete(ctx: Context, command: AutocompleteInteraction, media_type: MediaType) {
    let search = &command.data.options.first().unwrap().value;
    if let Some(search) = search {
        let query = search.as_str().unwrap_or_default();
        let result = anilist::query_titles(query, media_type, 8).await;

        if let Ok(result) = result {
            if let Some(res) = result.data {
                if let Some(media) = res.page.media {
                    let suggestions: Vec<AutocompleteOption> = media
                        .iter()
                        .filter_map(|item| {
                            item.as_ref().map(|item| AutocompleteOption {
                                    name: match &item.title {
                                        Some(title) => {
                                            String::from(title.english.as_ref().unwrap_or(
                                                title.romaji.as_ref().unwrap_or(&String::default()),
                                            ))
                                        }
                                        None => String::default(),
                                    },
                                    value: item.id.to_string(),
                                })
                        })
                        .collect();
                    let choices = json!(suggestions);

                    // doesn't matter if it errors
                    _ = command
                        .create_autocomplete_response(ctx.http, |response| {
                            response.set_choices(choices)
                        })
                        .await;
                }
            }
        }
    }
}

pub fn register(
    command: &mut CreateApplicationCommand,
    media_type: MediaType,
) -> &mut CreateApplicationCommand {
    match media_type {
        MediaType::Anime => command
            .name("anime")
            .description("Search anime on AniList")
            .create_option(|option| {
                option
                    .name("name")
                    .description("The name of the anime")
                    .kind(CommandOptionType::String)
                    .set_autocomplete(true)
                    .required(true)
            }),
        MediaType::Manga => command
            .name("manga")
            .description("Search manga on AniList")
            .create_option(|option| {
                option
                    .name("name")
                    .description("The name of the manga")
                    .kind(CommandOptionType::String)
                    .set_autocomplete(true)
                    .required(true)
            }),
    }
}
