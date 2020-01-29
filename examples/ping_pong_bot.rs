use teloxide::{
    dispatching::{
        session::{SessionDispatcher, SessionHandlerCtx, SessionState},
        Dispatcher,
    },
    requests::Request,
    types::Message,
    Bot,
};

#[tokio::main]
async fn main() {
    Dispatcher::<(), (), _, _, ()>::new(&Bot::new(
        "1061598315:AAErEDodTsrqD3UxA_EvFyEfXbKA6DT25G0",
    ))
    .private_message_dp(SessionDispatcher::new(
        |ctx: SessionHandlerCtx<Message, ()>| async move {
            ctx.bot
                .send_message(ctx.update.chat.id, "pong")
                .send()
                .await
                .unwrap();
            SessionState::Continue(())
        },
    ))
    .dispatch()
    .await
}
