use teloxide_core::{
    prelude::*,
    types::{DiceEmoji, Me, ParseMode},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let chat_id =
        ChatId(std::env::var("CHAT_ID").expect("Expected CHAT_ID env var").parse::<i64>()?);

    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2);

    let Me { user: me, .. } = bot.get_me().await?;

    bot.send_dice(chat_id).emoji(DiceEmoji::Dice).await?;
    bot.send_message(chat_id, format!("Hi, my name is **{}** 👋", me.first_name)).await?;

    Ok(())
}
