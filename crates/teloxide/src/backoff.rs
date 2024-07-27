use std::time::Duration;

pub type BackoffStrategy = Box<dyn Send + Fn(u32) -> Duration>;

const THRESHOLD_ERROR_QUANTITY: u32 = 6;

/// Calculates the backoff time in seconds for exponential strategy with base 2
///
/// The maximum duration is limited to a little more than a minute: 64s, so the
/// successive timings are: 1s, 2s, 4s, .., 64, .., 64
///
/// More at: <https://en.wikipedia.org/wiki/Exponential_backoff#Exponential_backoff_algorithm>
pub fn exponential_backoff_strategy(error_count: u32) -> Duration {
    // 2^6 = 64s ~ a little more than a minute
    Duration::from_secs(1_u64 << error_count.min(THRESHOLD_ERROR_QUANTITY))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, Duration::from_secs(2))]
    #[case(5, Duration::from_secs(32))]
    #[case(42, Duration::from_secs(64))]
    fn test_exponential_backoff_strategy(#[case] error_count: u32, #[case] expected: Duration) {
        assert_eq!(exponential_backoff_strategy(error_count), expected);
    }
}
