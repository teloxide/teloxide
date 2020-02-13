use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let bot = Bot::from_env().enable_logging(crate_name!()).build();
    log::info!("Starting multiple_handlers_bot!");

    // Create a dispatcher with multiple handlers of different types. This will
    // print One! and Two! on every incoming UpdateKind::Message.
    Dispatcher::<RequestError>::new(bot)
        // This is the first UpdateKind::Message handler, which will be called
        // after the Update handler below.
        .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
            log::info!("Two!");
            DispatcherHandlerResult::next(ctx.update, Ok(()))
        })
        // Remember: handler of Update are called first.
        .update_handler(&|ctx: DispatcherHandlerCtx<Update>| async move {
            log::info!("One!");
            DispatcherHandlerResult::next(ctx.update, Ok(()))
        })
        // This handler will be called right after the first UpdateKind::Message
        // handler, because it is registered after.
        .message_handler(&|_ctx: DispatcherHandlerCtx<Message>| async move {
            // The same as DispatcherHandlerResult::exit(Ok(()))
            Ok(())
        })
        // This handler will never be called, because the UpdateKind::Message
        // handler above terminates the pipeline.
        .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
            log::info!("This will never be printed!");
            DispatcherHandlerResult::next(ctx.update, Ok(()))
        })
        .dispatch()
        .await;

    // Note: if this bot receive, for example, UpdateKind::ChannelPost, it will
    // only print "One!", because the UpdateKind::Message handlers will not be
    // called.
}
