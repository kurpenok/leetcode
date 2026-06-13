pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    (left..=right).all(|x| ranges.iter().any(|r| r[0] <= x && x <= r[1]))
}
