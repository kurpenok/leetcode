use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    let mut numbers: HashMap<i32, usize> = HashMap::new();
    deck.iter().for_each(|number| {
        numbers
            .entry(*number)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });

    let mut values = Vec::new();
    for val in numbers.values() {
        values.push(*val);
    }

    let mut res = values[0];
    for i in 1..values.len() {
        res = gcd(res, values[i]);
    }

    res >= 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]), false);
    }
}
