use std::collections::HashMap;

fn calculate_total(n: i32) -> i32 {
    let mut total = 0;
    for num in format!("{}", n).chars() {
        total += num.to_digit(10).unwrap() as i32;
    }
    total
}

pub fn count_largest_group(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in 1..=n {
        let total = calculate_total(num);
        map.entry(total).and_modify(|x| *x += 1).or_insert(1);
    }

    let max_total = *map.values().max().unwrap();
    map.values().filter(|&&x| x == max_total).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(count_largest_group(13), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_largest_group(2), 2);
    }
}
