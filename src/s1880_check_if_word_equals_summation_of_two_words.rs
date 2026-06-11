pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
    let mut first_number = 0;
    let mut second_number = 0;
    let mut target_number = 0;

    for c in first_word.chars() {
        let digit = (c as u8 - b'a') as i32;
        first_number = first_number * 10 + digit;
    }

    for c in second_word.chars() {
        let digit = (c as u8 - b'a') as i32;
        second_number = second_number * 10 + digit;
    }

    for c in target_word.chars() {
        let digit = (c as u8 - b'a') as i32;
        target_number = target_number * 10 + digit;
    }

    first_number + second_number == target_number
}
