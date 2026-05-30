use std::collections::HashMap;

fn compute(expression: &str, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
    if let Some(result) = memo.get(expression) {
        return result.clone();
    }

    let mut results = Vec::new();

    for (i, c) in expression.chars().enumerate() {
        if c == '+' || c == '-' || c == '*' {
            let left = &expression[0..i];
            let right = &expression[i + 1..];

            let left_results = compute(left, memo);
            let right_results = compute(right, memo);

            for &l in &left_results {
                for &r in &right_results {
                    let val = match c {
                        '+' => l + r,
                        '-' => l - r,
                        '*' => l * r,
                        _ => unreachable!(),
                    };
                    results.push(val);
                }
            }
        }
    }

    if results.is_empty() {
        results.push(expression.parse::<i32>().unwrap());
    }

    memo.insert(expression.to_string(), results.clone());
    results
}

pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    let mut memo = HashMap::new();
    compute(&expression, &mut memo)
}
