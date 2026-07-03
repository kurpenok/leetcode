pub fn min_time_to_type(word: String) -> i32 {
    let mut seconds = 0;
    let mut current_seconds = 0;

    for c in word.chars() {
        let index = (c as u8 - b'a') as i32;
        let diff = (index - current_seconds).abs();
        let min_moves = diff.min(26 - diff);
        seconds += min_moves + 1;
        current_seconds = index;
    }

    seconds
}
