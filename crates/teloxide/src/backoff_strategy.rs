use std::time::Duration;

pub type BackoffStrategy = Box<dyn Fn(u32) -> Duration + Send>;

/// Calculates the backoff time in seconds for exponential strategy with base 2
///
/// More at: <https://en.wikipedia.org/wiki/Exponential_backoff#Exponential_backoff_algorithm>
pub fn exponential_backoff_strategy(error_count: u32) -> Duration {
    Duration::from_secs(2_u64.pow(error_count))
}
