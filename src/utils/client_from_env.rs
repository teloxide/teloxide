use crate::bot::{sound_bot, TELOXIDE_PROXY};

/// Constructs a client from the `TELOXIDE_PROXY` environmental variable.
///
/// This function passes the value of `TELOXIDE_PROXY` into
/// [`reqwest::Proxy::all`], if it exists, otherwise returns the default
/// client.
///
/// # Note
/// The created client will have safe settings, meaning that it will be able to
/// work in long time durations, see the [issue 223].
///
/// [`reqwest::Proxy::all`]: https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html#method.all
/// [issue 223]: https://github.com/teloxide/teloxide/issues/223
pub fn client_from_env() -> reqwest::Client {
    use reqwest::Proxy;

    let builder = sound_bot();

    match std::env::var(TELOXIDE_PROXY).ok() {
        Some(proxy) => builder.proxy(Proxy::all(&proxy).expect("creating reqwest::Proxy")),
        None => builder,
    }
    .build()
    .expect("creating reqwest::Client")
}
