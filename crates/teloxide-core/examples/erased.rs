use std::{env::VarError, time::Duration};

use teloxide_core::{adaptors::trace, prelude::*, types::ChatAction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let chat_id =
        ChatId(std::env::var("CHAT_ID").expect("Expected CHAT_ID env var").parse::<i64>()?);

    let trace_settings = match std::env::var("TRACE").as_deref() {
        Ok("EVERYTHING_VERBOSE") => trace::Settings::TRACE_EVERYTHING_VERBOSE,
        Ok("EVERYTHING") => trace::Settings::TRACE_EVERYTHING,
        Ok("REQUESTS_VERBOSE") => trace::Settings::TRACE_REQUESTS_VERBOSE,
        Ok("REQUESTS") => trace::Settings::TRACE_REQUESTS,
        Ok("RESPONSES_VERBOSE") => trace::Settings::TRACE_RESPONSES_VERBOSE,
        Ok("RESPONSES") => trace::Settings::TRACE_RESPONSES,
        Ok("EMPTY") | Ok("") | Err(VarError::NotPresent) => trace::Settings::empty(),
        Ok(_) | Err(VarError::NotUnicode(_)) => {
            panic!(
                "Expected `TRACE` environment variable to be equal to any of the following: \
                 `EVERYTHING_VERBOSE`, `EVERYTHING`, `REQUESTS_VERBOSE`, `REQUESTS`, \
                 `RESPONSES_VERBOSE`, `RESPONSES`, `EMPTY`, `` (empty string)"
            )
        }
    };

    log::info!("Trace settings: {trace_settings:?}");

    let bot = if trace_settings.is_empty() {
        Bot::from_env().erase()
    } else {
        Bot::from_env().trace(trace_settings).erase()
    };

    bot.send_chat_action(chat_id, ChatAction::Typing).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    bot.send_message(chat_id, "Hey hey hey").await?;

    Ok(())
}
