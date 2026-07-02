pub fn get_lucky(s: String, k: i32) -> i32 {
    let initial_sum = s
        .chars()
        .map(|c| {
            let x = ((c as u8 - b'a') + 1) as i32;
            (x / 10) + (x % 10)
        })
        .sum::<i32>();

    (0..k - 1).fold(initial_sum, |mut current, _| {
        let mut accumulator = 0;
        while current > 0 {
            accumulator += current % 10;
            current /= 10;
        }
        accumulator
    })
}
