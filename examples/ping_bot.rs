use futures::stream::StreamExt;
use teloxide::{
    dispatching::{
        chat::Dispatcher, update_listeners::polling_default, SessionState,
    },
    requests::Request,
    Bot,
};
use teloxide::dispatching::chat::{ChatUpdate, ChatUpdateKind};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = &Bot::new("1061598315:AAErEDodTsrqD3UxA_EvFyEfXbKA6DT25G0");
    let mut updater = Box::pin(polling_default(bot));
    let handler = |s, upd: ChatUpdate| async move {
        match upd.kind {
            ChatUpdateKind::Message(m) => {
                let msg = bot.send_message(m.chat.id, "pong");
                msg.send().await.unwrap();
            }
            _ => {}
        }
        SessionState::Continue(s)
    };
    let mut dp = Dispatcher::<'_, (), _>::new(handler);
    log::info!("Starting the message handler.");
    loop {
        let u = updater.next().await.unwrap();
        match u {
            Err(e) => log::error!("{}", e),
            Ok(u) => {
                let _ = dp.dispatch(u).await;
            }
        }
    }
}
