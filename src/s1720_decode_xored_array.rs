pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    encoded.iter().fold(vec![first], |mut decoded, &n| {
        decoded.push(decoded.last().unwrap() ^ n);
        decoded
    })
}
