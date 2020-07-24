/// Constructs a client from the `TELOXIDE_PROXY` environmental variable.
///
/// This function passes the value of `TELOXIDE_PROXY` into
/// [`reqwest::Proxy::all`], if it exists, otherwise returns the default
/// client.
///
/// [`reqwest::Proxy::all`]: https://docs.rs/reqwest/0.10.7/reqwest/struct.Proxy.html#method.all
pub fn client_from_env() -> reqwest::Client {
    use reqwest::{Client, Proxy};

    match std::env::var("TELOXIDE_PROXY").ok() {
        Some(proxy) => Client::builder()
            .proxy(Proxy::all(&proxy).expect("creating reqwest::Proxy"))
            .build()
            .expect("creating reqwest::Client"),
        None => Client::new(),
    }
}
