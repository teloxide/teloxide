use std::time::Duration;

pub type BackoffStrategy = Box<dyn Send + Fn(u32) -> Duration>;

/// Calculates the backoff time in seconds for exponential strategy with base 2
///
/// The maximum duration is limited to a little more than a minute: 64s, so the
/// successive timings are: 1s, 2s, 4s, .., 64, .., 64
///
/// More at: <https://en.wikipedia.org/wiki/Exponential_backoff#Exponential_backoff_algorithm>
pub fn exponential_backoff_strategy(error_count: u32) -> Duration {
    // 2^6 = 64s ~ a little more than a minute
    Duration::from_secs(1_u64 << error_count.min(6))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff_strategy() {
        let cases = [
            (1, Duration::from_secs(2)),
            (5, Duration::from_secs(32)),
            (42, Duration::from_secs(64)),
        ];

        for (error_count, expected) in cases {
            assert_eq!(exponential_backoff_strategy(error_count), expected);
        }
    }
}
