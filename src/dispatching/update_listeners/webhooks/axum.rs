use std::convert::Infallible;

use crate::{
    dispatching::{
        stop_token::StopToken,
        update_listeners::{
            webhooks::{setup_webhook, tuple_first_mut, Options},
            UpdateListener,
        },
    },
    requests::Requester,
};

/// Webhook implementation based on the [axum] framework.
///
/// ## Panics
///
/// If binding to the [address] fails.
///
/// [address]: Options.address
///
/// ## Errors
///
/// If `set_webhook()` fails.
pub async fn axum<R>(
    bot: R,
    options: Options,
) -> Result<impl UpdateListener<Infallible, StopToken = impl Send + StopToken>, R::Err>
where
    R: Requester + Send + 'static,
    <R as Requester>::DeleteWebhook: Send,
{
    let Options { address, .. } = options;

    let (mut update_listener, stop_flag, app) = axum_to_router(bot, options).await?;
    let stop_token = update_listener.stop_token();

    tokio::spawn(async move {
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .with_graceful_shutdown(stop_flag)
            .await
            .map_err(|err| {
                stop_token.stop();
                err
            })
            .expect("Axum server error");
    });

    Ok(update_listener)
}

pub async fn axum_to_router<R>(
    bot: R,
    mut options: Options,
) -> Result<
    (
        impl UpdateListener<Infallible, StopToken = impl Send + StopToken>,
        impl std::future::Future<Output = ()> + Send,
        axum::Router,
    ),
    R::Err,
>
where
    R: Requester + Send,
    <R as Requester>::DeleteWebhook: Send,
{
    use crate::requests::Request;
    use futures::FutureExt;

    setup_webhook(&bot, &mut options).await?;

    let (listener, stop_flag, router) = axum_no_setup(options);

    let stop_flag = stop_flag.then(move |()| async move {
        // This assignment is needed to not require `R: Sync` since without it `&bot`
        // temporary lives across `.await` points.
        let req = bot.delete_webhook().send();
        let res = req.await;
        if let Err(err) = res {
            log::error!("Couldn't delete webhook: {}", err);
        }
    });

    Ok((listener, stop_flag, router))
}

pub fn axum_no_setup(
    options: Options,
) -> (
    impl UpdateListener<Infallible, StopToken = impl Send + StopToken>,
    impl std::future::Future<Output = ()>,
    axum::Router,
) {
    use crate::{
        dispatching::{stop_token::AsyncStopToken, update_listeners},
        types::Update,
    };
    use axum::{
        extract::Extension, http::StatusCode, response::IntoResponse, routing::post,
        AddExtensionLayer,
    };
    use tokio::sync::mpsc;
    use tokio_stream::wrappers::UnboundedReceiverStream;
    use tower::ServiceBuilder;
    use tower_http::trace::TraceLayer;

    type Sender = mpsc::UnboundedSender<Result<Update, std::convert::Infallible>>;

    let (tx, rx): (Sender, _) = mpsc::unbounded_channel();

    async fn telegram_request(input: String, tx: Extension<Sender>) -> impl IntoResponse {
        match serde_json::from_str(&input) {
            Ok(update) => {
                tx.send(Ok(update)).expect("Cannot send an incoming update from the webhook")
            }
            Err(error) => {
                log::error!(
                    "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                     This is a bug in teloxide-core, please open an issue here: \
                     https://github.com/teloxide/teloxide/issues.",
                    error,
                    input
                );
            }
        };

        StatusCode::OK
    }

    let app = axum::Router::new().route(options.url.path(), post(telegram_request)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(tx))
            .into_inner(),
    );

    let (stop_token, stop_flag) = AsyncStopToken::new_pair();

    let stream = UnboundedReceiverStream::new(rx);

    let listener = update_listeners::StatefulListener::new(
        (stream, stop_token),
        tuple_first_mut,
        |state: &mut (_, AsyncStopToken)| state.1.clone(),
    );

    (listener, stop_flag, app)
}
