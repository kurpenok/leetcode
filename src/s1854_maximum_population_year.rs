pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
    let offset = logs.iter().map(|log| log[0]).min().unwrap();
    let size = logs.iter().map(|log| log[1]).max().unwrap() - offset + 1;
    let mut delta = vec![0; size as usize];

    for log in logs {
        delta[(log[0] - offset) as usize] += 1;
        delta[(log[1] - offset) as usize] -= 1;
    }

    let mut current = 0;
    let mut max_population = 0;
    let mut max_year = 0;

    for (i, d) in delta.iter().enumerate() {
        current += d;
        if max_population < current {
            max_population = current;
            max_year = i;
        }
    }

    offset + max_year as i32
}
