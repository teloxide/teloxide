//!
use std::net::SocketAddr;

use teloxide_core::types::AllowedUpdate;

use crate::{requests::Requester, types::InputFile};

/// Options related to setting up webhooks.
#[must_use]
pub struct Options {
    /// Local address to listen to.
    pub address: SocketAddr,

    /// Public url that Telegram will send updates to.
    ///
    /// Note:
    /// - At the time of writing only ports 443, 80, 88 and 8443 [are
    ///   supported][set_webhook]
    /// - This url must be forwarded to the [address][addr] in order for webhook
    ///   to work
    /// - This url should be kept private, otherwise malicious actors can
    ///   pretend to be Telegram and send fake updates to your bot
    ///
    /// [set_webhook]: https://core.telegram.org/bots/api#setwebhook
    /// [addr]: (self::Options.address)
    pub url: url::Url,

    /// Server-internal path to listen for requests on.
    ///
    /// This can differ from the path in `url` when you use a reverse proxy.
    ///
    /// Default - the URL path is reused.
    pub path: String,

    /// Upload your public key certificate so that the root certificate in use
    /// can be checked. See Telegram's [self-signed guide] for details.
    ///
    /// [self-signed guide]: https://core.telegram.org/bots/self-signed
    ///
    /// Default - None.
    pub certificate: Option<InputFile>,

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook
    /// for update delivery, 1-100. Defaults to 40. Use lower values to limit
    /// the load on your bot's server, and higher values to increase your bot's
    /// throughput.
    ///
    /// Default - None.
    pub max_connections: Option<u8>,

    /// A JSON-serialized list of the update types you want your bot to receive.
    /// For example, specify [“message”, “edited_channel_post”,
    /// “callback_query”] to only receive updates of these types. See [`Update`]
    /// for a complete list of available update types. Specify an empty list to
    /// receive all updates regardless of type (default). If not specified, the
    /// previous setting will be used.
    ///
    /// Please note that this parameter doesn't affect updates created before
    /// the call to the setWebhook, so unwanted updates may be received for a
    /// short period of time.
    ///
    /// [`Update`]: crate::types::Update
    pub allowed_updates: Option<Vec<AllowedUpdate>>,

    /// Pass `true` to drop all pending updates.
    ///
    /// Default - false.
    pub drop_pending_updates: bool,

    /// A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token”
    /// in every webhook request, 1-256 characters. Only characters `A-Z`,
    /// `a-z`, `0-9`, `_` and `-` are allowed. The header is useful to ensure
    /// that the request comes from a webhook set by you.
    ///
    /// Default - `teloxide` will generate a random token.
    pub secret_token: Option<String>,
}

impl Options {
    /// Construct a new webhook options, see [`Options::address`] and
    /// [`Options::url`] for details.
    pub fn new(address: SocketAddr, url: url::Url) -> Self {
        let path = url.path().to_owned();
        Self {
            address,
            url,
            path,
            certificate: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: false,
            secret_token: None,
        }
    }

    /// Specify a custom routing path. This can be useful when the server is
    /// behind a reverse proxy. By default, the path will be taken from the
    /// public URL.
    pub fn path(self, path: String) -> Self {
        Self { path, ..self }
    }

    /// Upload your public key certificate so that the root certificate in use
    /// can be checked. See Telegram's [self-signed guide] for details.
    ///
    /// [self-signed guide]: https://core.telegram.org/bots/self-signed
    pub fn certificate(self, v: InputFile) -> Self {
        Self { certificate: Some(v), ..self }
    }

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook
    /// for update delivery, 1-100. Defaults to 40. Use lower values to limit
    /// the load on your bot's server, and higher values to increase your bot's
    /// throughput.
    pub fn max_connections(self, v: u8) -> Self {
        Self { max_connections: Some(v), ..self }
    }

    /// Allowed updates.
    pub fn allowed_updates(self, v: Vec<AllowedUpdate>) -> Self {
        Self { allowed_updates: Some(v), ..self }
    }

    /// Drop all pending updates before setting up webhook.
    pub fn drop_pending_updates(self) -> Self {
        Self { drop_pending_updates: true, ..self }
    }

    /// A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token”
    /// in every webhook request, 1-256 characters. Only characters `A-Z`,
    /// `a-z`, `0-9`, `_` and `-` are allowed. The header is useful to ensure
    /// that the request comes from a webhook set by you.
    ///
    /// ## Panics
    ///
    /// If the token is invalid.
    #[track_caller]
    pub fn secret_token(self, token: String) -> Self {
        check_secret(token.as_bytes()).expect("Invalid secret token");

        Self { secret_token: Some(token), ..self }
    }

    /// Returns `self.secret_token`, generating a new one if it's `None`.
    ///
    /// After a call to this function `self.secret_token` is always `Some(_)`.
    ///
    /// **Note**: if you leave webhook setup to teloxide, it will automatically
    /// generate a secret token. Call this function only if you need to know the
    /// secret (for example because you are calling `set_webhook` by yourself).
    pub fn get_or_gen_secret_token(&mut self) -> &str {
        self.secret_token.get_or_insert_with(gen_secret_token)
    }
}

#[cfg(feature = "webhooks-axum")]
pub use self::axum::{axum, axum_no_setup, axum_to_router};

#[cfg(feature = "webhooks-axum")]
mod axum;

// TODO: add different implementation (for example: warp)

/// Calls `set_webhook` with arguments from `options`.
///
/// Note: this takes out `certificate`.
async fn setup_webhook<R>(bot: R, options: &mut Options) -> Result<(), R::Err>
where
    R: Requester,
{
    use crate::requests::Request;
    use teloxide_core::requests::HasPayload;

    let secret = options.get_or_gen_secret_token().to_owned();
    let &mut Options {
        ref url,
        ref mut certificate,
        max_connections,
        ref allowed_updates,
        drop_pending_updates,
        ..
    } = options;

    let mut req = bot.set_webhook(url.clone());
    req.payload_mut().certificate = certificate.take();
    req.payload_mut().max_connections = max_connections;
    req.payload_mut().allowed_updates = allowed_updates.clone();
    req.payload_mut().drop_pending_updates = Some(drop_pending_updates);
    req.payload_mut().secret_token = Some(secret);

    req.send().await?;

    Ok(())
}

/// Generates a random string consisting of 32 characters (`a-z`, `A-Z`, `0-9`,
/// `_` and `-`).
fn gen_secret_token() -> String {
    use rand::{distributions::Uniform, Rng};
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";
    const SECRET_LENGTH: usize = 32;

    let random = rand::thread_rng()
        .sample_iter(Uniform::new(0, CHARSET.len()))
        .map(|idx| CHARSET[idx] as char)
        .take(SECRET_LENGTH);

    let mut secret = String::with_capacity(SECRET_LENGTH);
    secret.extend(random);

    secret
}

fn check_secret(bytes: &[u8]) -> Result<&[u8], &'static str> {
    let len = bytes.len();

    if !(1..=256).contains(&len) {
        return Err("secret token length must be in range 1..=256");
    }

    let is_not_supported =
        |c: &_| !matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-');
    if bytes.iter().any(is_not_supported) {
        return Err("secret token must only contain of `a-z`, `A-Z`, `0-9`, `_` and `-` characters");
    }

    Ok(bytes)
}

/// Returns first (`.0`) field from a tuple as a `&mut` reference.
///
/// This hack is needed because there isn't currently a way to easily force a
/// closure to be higher-ranked (`for<'a> &'a mut _ -> &'a mut _`) which causes
/// problems when using [`StatefulListener`] to implement update listener.
///
/// This could be probably removed once [rfc#3216] is implemented.
///
/// [`StatefulListener`]:
/// [rfc#3216]: https://github.com/rust-lang/rfcs/pull/3216
fn tuple_first_mut<A, B>(tuple: &mut (A, B)) -> &mut A {
    &mut tuple.0
}
