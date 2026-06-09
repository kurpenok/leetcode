pub fn count_good_substrings(s: String) -> i32 {
    s.as_bytes()
        .windows(3)
        .filter(|w| w[0] != w[1] && w[0] != w[2] && w[1] != w[2])
        .count() as i32
}
