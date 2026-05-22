fn digit_sum(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut counts = [0; 50];

    for ball in low_limit..=high_limit {
        let sum = digit_sum(ball);
        counts[sum as usize] += 1;
    }

    *counts.iter().max().unwrap()
}
