pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    keys_pressed
        .chars()
        .zip(std::iter::once(release_times[0]).chain(release_times.windows(2).map(|w| w[1] - w[0])))
        .max_by_key(|&(symbol, duration)| (duration, symbol))
        .unwrap()
        .0
}
