use futures::stream::StreamExt;
use teloxide::{
    dispatching::{
        private::Dispatcher, update_listeners::polling_default, SessionState,
    },
    requests::Request,
    types::{Update, UpdateKind},
    Bot,
};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = &Bot::new("1061598315:AAErEDodTsrqD3UxA_EvFyEfXbKA6DT25G0");
    let mut updater = Box::pin(polling_default(bot));
    let handler = |s, upd: Update| async move {
        match upd.kind {
            UpdateKind::Message(m) => {
                let msg = bot.send_message(m.chat.id, "pong");
                msg.send().await.unwrap();
            }
            _ => {}
        }
        SessionState::Continue(s)
    };
    let mut dp = Dispatcher::<'_, (), _>::new(handler);
    info!("Starting the message handler.");
    loop {
        let u = updater.next().await.unwrap();
        match u {
            Err(e) => error!("{}", e),
            Ok(u) => {
                let _ = dp.dispatch(u).await;
            }
        }
    }
}
