pub fn count_triples(n: i32) -> i32 {
    (1..=n)
        .flat_map(|a| {
            let a_square = a.pow(2);
            (1..=n)
                .map(move |b| {
                    let sum_squares = a_square + b.pow(2);
                    let c = (sum_squares as f64).sqrt() as i32;
                    (sum_squares, c)
                })
                .take_while(|&(_, c)| c <= n)
                .filter(|&(sum_squares, c)| c.pow(2) == sum_squares)
        })
        .count() as i32
}
