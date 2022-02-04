//! This example provide quick overview of the new features in the
//! `dispatching2` module.

use rand::Rng;
/// Note that you need to import `prelude2` because `prelude` contains
/// items from the old dispatching system (it may change in future versions).
use teloxide::prelude2::*;
use teloxide::{types::Update, utils::command::BotCommand};

#[tokio::main]
async fn main() {
    // Start you main as early: start logging, create bot, etc.
    teloxide::enable_logging!();
    log::info!("Starting dispatching2_features_bot...");
    let bot = Bot::from_env().auto_send();

    let parameters = ConfigParameters {
        bot_maintainer: 268486177, // Paste your ID if you run this bot.
        maintainer_username: None,
    };

    let handler = Update::filter_message()
        // Branch is a special method that allow you to handle update several ways.
        .branch(
            // Filter allow you to filter updates by some condition.
            dptree::filter(
                |msg: Message| msg.chat.is_group() || msg.chat.is_supergroup(),
            )
            // Endpoint is a last message handler.
            .endpoint(|msg: Message, bot: AutoSend<Bot>| async move {
                log::info!("Received message from the group chat.");
                bot.send_message(msg.chat.id, "This is a group chat.").await?;
                respond(())
            }),
        )
        // Note that we cannot filter messages from public chats in the next branch,
        // because they all is handled by previous branch.
        .branch(
            // There are some `filter` functions on message, that filters events. This
            // filter will filter only messages with dices.
            Message::filter_dice().endpoint(|msg: Message, bot: AutoSend<Bot>| async move {
                bot.send_message(msg.chat.id, "This is a dice!")
                    .reply_to_message_id(msg.id)
                    .await?;
                Ok(())
            }),
        )
        .branch(
            // If you do not like photos, you can break their handling like that.
            Message::filter_photo().endpoint(|| async move { Ok(()) }),
        )
        .branch(
            dptree::entry()
                // This method allows to parse text messages commands.
                .filter_command::<SimpleCommand>()
                // Next we can add `SimpleCommand` in the argument of endpoint. If
                // command parsing fails, this endpoint will not be called.
                .endpoint(simple_commands_handler),
        )
        .branch(
            // Filter maintainer by used ID.
            dptree::filter(|msg: Message, cfg: ConfigParameters| {
                msg.from().map(|user| user.id == cfg.bot_maintainer).unwrap_or_default()
            })
            .filter_command::<MaintainerCommands>()
            .endpoint(
                |msg: Message, bot: AutoSend<Bot>, cmd: MaintainerCommands| async move {
                    match cmd {
                        MaintainerCommands::Rand { from, to } => {
                            let mut rng = rand::rngs::OsRng::default();
                            let value: u64 = rng.gen_range(from..=to);

                            bot.send_message(msg.chat.id, value.to_string()).await?;

                            Ok(())
                        }
                    }
                },
            ),
        );

    // Start create dispatcher.
    Dispatcher::builder(bot, handler)
        // You can specify dependencies to that you have access inside of handlers. It may be
        // configs, connection to Database, or dialogue storage (see more in the dialogue_bot
        // example). It is similar to the `actix_web::Extensions`.
        .dependencies(dptree::deps![parameters])
        // Now handlers don't use streams. Instead handler is special constructs from `dptree`
        // library. Any `*_handler` accepts function `Fn(UpdateHandler) -> UpdateHandler`
        // which is builder for the handlers. Note that you _must_ use it instead of using
        // `dptree` methods forward.
        .default_handler(|upd| async move {
            // This handler handles updates that do not handled by other handlers.
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If `Result::Err` returns from the dispatcher, it goes here.
        .error_handler(LoggingErrorHandler::with_custom_text(
            "Error has occurred in the dispatcher",
        ))
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

#[derive(Clone)]
struct ConfigParameters {
    bot_maintainer: i64,
    maintainer_username: Option<String>,
}

// We do not change BotCommand api.
#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "Simple commands")]
enum SimpleCommand {
    #[command(description = "shows this message.")]
    Help,
    #[command(description = "shows maintainer info.")]
    Maintainer,
    #[command(description = "shows your ID.")]
    MyId,
}

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "Maintainer commands")]
enum MaintainerCommands {
    #[command(parse_with = "split", description = "generate a number within range")]
    Rand { from: u64, to: u64 },
}

async fn simple_commands_handler(
    msg: Message,
    bot: AutoSend<Bot>,
    cmd: SimpleCommand,
    cfg: ConfigParameters,
) -> Result<(), teloxide::RequestError> {
    let text = match cmd {
        SimpleCommand::Help => {
            if msg.from().unwrap().id == cfg.bot_maintainer {
                format!("{}\n{}", SimpleCommand::descriptions(), MaintainerCommands::descriptions())
            } else {
                SimpleCommand::descriptions()
            }
        }
        SimpleCommand::Maintainer => {
            if msg.from().unwrap().id == cfg.bot_maintainer {
                "Maintainer is you!".into()
            } else if let Some(username) = cfg.maintainer_username {
                format!("Maintainer is @{}", username)
            } else {
                format!("Maintainer ID is {}", cfg.bot_maintainer)
            }
        }
        SimpleCommand::MyId => {
            format!("{}", msg.from().unwrap().id)
        }
    };
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}
