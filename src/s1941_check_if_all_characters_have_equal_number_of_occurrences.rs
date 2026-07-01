pub fn are_occurrences_equal(s: String) -> bool {
    let mut frequencies = [0; 26];

    for c in s.chars() {
        frequencies[(c as u8 - b'a') as usize] += 1;
    }

    let mut iter = frequencies.iter().filter(|&&f| f != 0);
    if let Some(&first_non_zero_val) = iter.next() {
        iter.all(|&f| f == first_non_zero_val)
    } else {
        true
    }
}
