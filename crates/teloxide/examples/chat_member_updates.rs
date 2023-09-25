use teloxide::{
    dispatching::Dispatcher,
    prelude::*,
    types::{ChatMemberUpdated, ParseMode, Update},
    utils::html,
};
use teloxide_core::adaptors::DefaultParseMode;

/// We need the Bot alias type because we need use the
/// `ParseMode::Html` by default
/// (This is optional for this example, use the `parse_mode` method
/// for set the parse mode in a specific message)
type Bot = DefaultParseMode<teloxide::Bot>;

#[tokio::main]
async fn main() -> ResponseResult<()> {
    pretty_env_logger::init();

    // We need use `ParseMode::Html` by default because the `first_name` or
    // `username` is formatted from `<a href="tg://user?
    // id={user_id}>{username}</a>"` to a message with link in the
    // chat_member_handler function
    let bot = teloxide::Bot::from_env().parse_mode(ParseMode::Html);

    // We create a handler for our bot
    let handler = dptree::entry()
        .inspect(|u: Update| {
            println!("{u:#?}"); // Print the update to the console with inspect
                                // method and a closure for debug purposes
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

    // We create a dispatcher for our bot
    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;

    Ok(())
}

/// Welcome Function
/// We use ChatMemberUpdated instead of Message for our function because
/// Chat member updates != messages
async fn new_chat_member(bot: Bot, chat_member: ChatMemberUpdated) -> ResponseResult<()> {
    // We use this variable for get the user
    let user = chat_member.old_chat_member.user.clone();

    // We use this variable for get the user_id
    let user_id = user.id;

    // We use this variable for get the group name
    let telegram_group_name = chat_member.chat.title().unwrap_or("");

    // We get the full_name of the user via `mention()` method and we use
    // `unwrap_or_else` for get the first_name via `full_name` method
    // if the user don't have a username
    let username =
        user.mention().unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

    // If the user is present, we send a welcome message
    bot.send_message(chat_member.chat.id, format!("Welcome to {telegram_group_name} {username}!"))
        .await?;

    Ok(())
}

async fn left_chat_member(bot: Bot, chat_member: ChatMemberUpdated) -> ResponseResult<()> {
    // We use this variable for get the user
    let user = chat_member.old_chat_member.user;

    // We use this variable for get the user_id
    let user_id = user.id;

    // We get the full_name of the user via `mention()` method and we use
    // `unwrap_or_else` for get the first_name via `full_name` method
    // if the user don't have a username
    let username =
        user.mention().unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

    // If the user is gone, we send a goodbye message
    bot.send_message(chat_member.chat.id, format!("Goodbye {username}!")).await?;

    Ok(())
}
