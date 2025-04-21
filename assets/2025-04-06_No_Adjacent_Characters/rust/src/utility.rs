use std::time::{Duration, Instant};

pub fn is_valid_vector(vector: &[u128]) -> bool {
    !vector.windows(2).any(|w| w[0] == w[1])
}

pub fn measure<F: FnOnce()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

/// Stats help
pub fn average(data: &[u128]) -> u128 {
    if data.is_empty() { 0 } else { data.iter().sum::<u128>() / data.len() as u128 }
}

pub fn min(data: &[u128]) -> u128 {
    *data.iter().min().unwrap_or(&0)
}

pub fn max(data: &[u128]) -> u128 {
    *data.iter().max().unwrap_or(&0)
}

pub fn std_dev(data: &[u128]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let mean = average(data) as f64;
    let variance = data.iter()
        .map(|&val| {
            let diff = val as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / (data.len() - 1) as f64;

    variance.sqrt()
}
