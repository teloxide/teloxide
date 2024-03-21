use teloxide::{prelude::*, types::ParseMode, utils::html};
use teloxide_core::adaptors::DefaultParseMode;

/// We use a type alias to be able to write just `bot: Bot` in handlers, instead
/// of a lengthy `DefaultParseMode<Bot>`.
///
/// `DefaultParseMode` here is a [requester adaptor], that allows us to specify
/// a default parse mode. The other option would be to use `.parse_mode(...)`
/// after each API call manually (e.g.
/// `bot.send_message(...).parse_mode(...).await?`).
///
/// [requester adaptor]: teloxide::requests::Requester#adaptors
type Bot = DefaultParseMode<teloxide::Bot>;

#[tokio::main]
async fn main() -> ResponseResult<()> {
    pretty_env_logger::init();

    // We specify default parse mode to be `Html`, so that later we can use
    // `html::user_mention`
    let bot = teloxide::Bot::from_env().parse_mode(ParseMode::Html);

    // Create a handler for our bot, that will process updates from Telegram
    let handler = dptree::entry()
        .inspect(|u: Update| {
            eprintln!("{u:#?}"); // Print the update to the console with inspect
        })
        .branch(
            Update::filter_chat_member()
                .branch(
                    dptree::filter(|m: ChatMemberUpdated| {
                        m.old_chat_member.is_left() && m.new_chat_member.is_present()
                    })
                    .endpoint(new_chat_member),
                )
                .branch(
                    dptree::filter(|m: ChatMemberUpdated| {
                        m.old_chat_member.is_present() && m.new_chat_member.is_left()
                    })
                    .endpoint(left_chat_member),
                ),
        );

    // Create a dispatcher for our bot
    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;

    Ok(())
}

/// Welcome Endpoint
async fn new_chat_member(bot: Bot, chat_member: ChatMemberUpdated) -> ResponseResult<()> {
    let user = chat_member.old_chat_member.user.clone();

    let telegram_group_name = chat_member.chat.title().unwrap_or("");

    // We get a "@username" mention via `mention()` method if the user has a
    // username, otherwise we create a textual mention with "Full Name" as the
    // text linking to the user
    let username =
        user.mention().unwrap_or_else(|| html::user_mention(user.id, user.full_name().as_str()));

    bot.send_message(chat_member.chat.id, format!("Welcome to {telegram_group_name} {username}!"))
        .await?;

    Ok(())
}

async fn left_chat_member(bot: Bot, chat_member: ChatMemberUpdated) -> ResponseResult<()> {
    let user = chat_member.old_chat_member.user;

    let username =
        user.mention().unwrap_or_else(|| html::user_mention(user.id, user.full_name().as_str()));

    bot.send_message(chat_member.chat.id, format!("Goodbye {username}!")).await?;

    Ok(())
}
