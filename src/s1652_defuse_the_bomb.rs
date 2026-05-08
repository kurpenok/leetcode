pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();

    match k {
        0 => vec![0; n],
        _ => (0..n)
            .map(|i| {
                (1..=k.abs() as usize)
                    .map(|j| code[(i as i32 + k.signum() * j as i32).rem_euclid(n as i32) as usize])
                    .sum()
            })
            .collect(),
    }
}
