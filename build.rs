fn main() {
    println!(
        "cargo:rustc-env=TELEGRAM_API_URL={}",
        option_env!("TELEGRAM_API_URL").unwrap_or("https://api.telegram.org")
    );
}
