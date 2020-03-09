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
