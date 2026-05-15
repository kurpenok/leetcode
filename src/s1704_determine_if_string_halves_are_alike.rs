pub fn halves_are_alike(s: String) -> bool {
    s[..s.len() / 2]
        .chars()
        .filter(|&c| "aeiouAEIOU".contains(c))
        .count()
        == s[s.len() / 2..]
            .chars()
            .filter(|&c| "aeiouAEIOU".contains(c))
            .count()
}
