pub fn check_if_pangram(sentence: String) -> bool {
    sentence
        .bytes()
        .fold(0u32, |mask, b| mask | 1 << (b - b'a'))
        == (1 << 26) - 1
}
