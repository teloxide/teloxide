//!
use std::net::SocketAddr;

use crate::{requests::Requester, types::InputFile};

/// Options related to setting up webhooks.
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

    /// Pass `true` to drop all pending updates.
    ///
    /// Default - false.
    pub drop_pending_updates: bool,
}

impl Options {
    /// Construct a new webhook options, see [`Options::address`] and
    /// [`Options::url`] for details.
    pub fn new(address: SocketAddr, url: url::Url) -> Self {
        Self { address, url, certificate: None, max_connections: None, drop_pending_updates: false }
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

    /// Drop all pending updates before setting up webhook.
    pub fn drop_pending_updates(self) -> Self {
        Self { drop_pending_updates: true, ..self }
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

    let &mut Options {
        ref url, ref mut certificate, max_connections, drop_pending_updates, ..
    } = options;

    let mut req = bot.set_webhook(url.clone());
    req.payload_mut().certificate = certificate.take();
    req.payload_mut().max_connections = max_connections;
    req.payload_mut().drop_pending_updates = Some(drop_pending_updates);

    req.send().await?;

    Ok(())
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
