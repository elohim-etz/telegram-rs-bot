use teloxide::prelude::*;
use log::{info, error};
use crate::bot::{
    commands::Command, 
    responses::*,
    quotes::get_random_quote,
    kawaii::get_kawaii_image,
    percent::get_daily_percentage,
    advice::get_daily_advice,
    waifu::{get_waifu_image, WaifuError},
    weather::{get_weather, WeatherError}
};
use teloxide::types::{InputFile};
use url::Url;

pub async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + >> {
    info!("Received command: {:?} from {:?}", cmd, msg.chat.id);
    
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::help_text()).await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, welcome_message()).await?;
        }
        Command::Random => {
            let num = rand::random::<u8>();
            bot.send_message(msg.chat.id, format!("Your random number: {}", num))
                .await?;
        }
        Command::Echo(text) => {
            bot.send_message(msg.chat.id, text).await?;
        }
        Command::RandomQuote => {
            match get_random_quote().await {
                Ok(quote) => {
                    let alt_name = quote.anime.alt_name
                        .map(|name| format!(" / {}", name))
                        .unwrap_or_default();
                    
                    let text = format!(
                        "\"{}\"\n— {} ({} {})",
                        quote.content,
                        quote.character.name,
                        quote.anime.name,
                        alt_name
                    );
                    bot.send_message(msg.chat.id, text).await?;
                }
                Err(e) => {
                    error!("Failed to fetch quote: {}", e);
                    bot.send_message(msg.chat.id, "Failed to get a quote 😢").await?;
                }
            }
        }
        Command::Kawaii(category) => {
            match get_kawaii_image(&category).await {
                Ok(image_url) => {
                    bot.send_photo(msg.chat.id, teloxide::types::InputFile::url(image_url))
                        .await?;
                }
                Err(e) => {
                    log::error!("Failed to fetch kawaii image: {}", e);
                    let help_text = "Usage: /kawaii <category>\n\nValid categories: waifu, neko, shinobu, megumin, bully, cuddle, cry, hug, awoo, kiss, lick, pat, smug, bonk, yeet, blush, smile, wave, highfive, handhold, nom, bite, glomp, slap, kill, kick, happy, wink, poke, dance, cringe";
                    bot.send_message(msg.chat.id, help_text).await?;
                }
            }
        }
        Command::Sexy => {
            let target_user = if let Some(replied_to) = msg.reply_to_message() {
                replied_to.from().unwrap()
            } else {
                msg.from().unwrap()
            };
        
            let percent = get_daily_percentage(target_user, "sexy");
            let first_name = &target_user.first_name;
            
            bot.send_message(
                msg.chat.id,
                format!("Today <a href=\"tg://user?id={}\">{}</a> is {}% sexy! 😘", 
                       target_user.id.0, first_name, percent)
            ).parse_mode(teloxide::types::ParseMode::Html)
             .await?;
        },
        Command::Gay => {
            let target_user = if let Some(replied_to) = msg.reply_to_message() {
                replied_to.from().unwrap()
            } else {
                msg.from().unwrap()
            };
        
            let percent = get_daily_percentage(target_user, "gay");
            let first_name = &target_user.first_name;
            
            bot.send_message(
                msg.chat.id,
                format!("Today <a href=\"tg://user?id={}\">{}</a> is {}% gay! 🌈", 
                       target_user.id.0, first_name, percent)
            ).parse_mode(teloxide::types::ParseMode::Html)
             .await?;
        },
        Command::Advice => {
            let target_user = if let Some(replied_to) = msg.reply_to_message() {
                replied_to.from().unwrap()
            } else {
                msg.from().unwrap()
            };
        
            match get_daily_advice().await {
                Ok(advice) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("Dear <a href=\"tg://user?id={}\">{}</a>, here's your advice:\n\"{}\"", 
                              target_user.id.0, target_user.first_name, advice)
                    ).parse_mode(teloxide::types::ParseMode::Html)
                     .await?;
                }
                Err(e) => {
                    log::error!("Failed to fetch advice: {}", e);
                    bot.send_message(
                        msg.chat.id,
                        "Sorry, I couldn't get any advice right now 😢"
                    ).await?;
                }
            }
        },
        Command::Waifu(tag) => {
            match get_waifu_image(&tag).await {
                Ok((image_url, caption, document_url)) => {
                    // Send normal image with caption
                    let image = InputFile::url(Url::parse(&image_url)?);
                    bot.send_photo(msg.chat.id, image)
                        .caption(caption)
                        .await?;
        
                    // Send same image as document (lossless)
                    let document = InputFile::url(Url::parse(&document_url)?);
                    bot.send_document(msg.chat.id, document)
                        .await?;
                }
                Err(e) => {
                    bot.send_message(
                        msg.chat.id,
                        match e {
                            WaifuError::NoImages => "No images found with that tag",
                            _ => "Failed to fetch waifu image",
                        }
                    ).await?;
                }
            }
        },
        Command::Weather(location) => {
            match get_weather(&location).await {
                Ok(weather) => {
                    let response = format!(
                        "🌦 Weather in {}, {}, {}:\n\
                        🕒 Local Time: {}\n\
                        🌡 Temperature: {:.1}°C (Feels like {:.1}°C)\n\
                        🌬 Wind: {:.1} km/h\n\
                        💧 Humidity: {}%\n\
                        ☁ Condition: {}",
                        weather.location.name,
                        weather.location.region,
                        weather.location.country,
                        weather.location.localtime,
                        weather.current.temp_c,
                        weather.current.feelslike_c,
                        weather.current.wind_kph,
                        weather.current.humidity,
                        weather.current.condition.text
                    );
        
                    bot.send_message(msg.chat.id, response).await?;
                }
                Err(e) => {
                    bot.send_message(
                        msg.chat.id,
                        match e {
                            WeatherError::LocationNotFound => "Location not found. Try another one.",
                            _ => "Failed to fetch weather data.",
                        }
                    ).await?;
                }
            }
        },
    }

    Ok(())
}

pub async fn handle_messages(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        info!("Received message: {} from {:?}", text, msg.chat.id);
        
        // React to certain words
        if text.to_lowercase().contains("hello") {
            bot.send_message(msg.chat.id, "Hi there! 👋").await?;
        } else if text.to_lowercase().contains("thanks") {
            bot.send_message(msg.chat.id, "You're welcome! 😊").await?;
        } else if text.to_lowercase().contains("bye") {
            bot.send_message(msg.chat.id, "Goodbye! 👋").await?;
        }
    }

    Ok(())
}