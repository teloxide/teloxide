use std::time::Duration;

pub type BackoffStrategy = Box<dyn Send + Fn(u32) -> Duration>;

/// Calculates the backoff time in seconds for exponential strategy with base 2
///
/// The maximum duration is limited to a little less than half an hour (1024
/// secs), so the successive timings are(in secs): 1, 2, 4, .., 1024, 1024, ..
///
/// More at: <https://en.wikipedia.org/wiki/Exponential_backoff#Exponential_backoff_algorithm>
pub fn exponential_backoff_strategy(error_count: u32) -> Duration {
    // The error_count has to be limited so as not to cause overflow: 2^10 = 1024 ~
    // a little less than half an hour
    Duration::from_secs(1_u64 << error_count.min(10))
}
