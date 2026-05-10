pub fn max_repeating(sequence: String, word: String) -> i32 {
    let mut k = sequence.len() / word.len();

    while k > 0 {
        if sequence.contains(&word.repeat(k)) {
            return k as i32;
        }
        k -= 1;
    }

    0
}
