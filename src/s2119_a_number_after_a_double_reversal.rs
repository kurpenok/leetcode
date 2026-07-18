pub fn is_same_after_reversals(num: i32) -> bool {
    if num % 10 == 0 && num != 0 {
        false
    } else {
        true
    }
}
