pub fn check_zero_ones(s: String) -> bool {
    let mut max_ones = 0;
    let mut max_zeros = 0;
    let mut current_ones = 0;
    let mut current_zeros = 0;

    for c in s.chars() {
        if c == '1' {
            current_ones += 1;
            current_zeros = 0;
            max_ones = max_ones.max(current_ones);
        } else {
            current_zeros += 1;
            current_ones = 0;
            max_zeros = max_zeros.max(current_zeros);
        }
    }

    max_ones > max_zeros
}
