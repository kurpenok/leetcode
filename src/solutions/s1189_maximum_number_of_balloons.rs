use std::collections::HashMap;

pub fn max_number_of_balloons(text: String) -> i32 {
    let balloon_map: HashMap<char, usize> =
        HashMap::from([('b', 1), ('a', 1), ('l', 2), ('o', 2), ('n', 1)]);
    let text_map: HashMap<char, usize> = text.chars().fold(HashMap::new(), |mut map, c| {
        let _ = *map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        map
    });

    let mut balloon_counter: HashMap<char, usize> =
        HashMap::from([('b', 0), ('a', 0), ('l', 0), ('o', 0), ('n', 0)]);
    for (symbol, count) in &text_map {
        if balloon_map.contains_key(symbol) {
            let value = count / balloon_map[symbol];
            balloon_counter
                .entry(*symbol)
                .and_modify(|count| *count = value)
                .or_insert(value);
        }
    }

    if let Some(min_value) = balloon_counter.values().copied().min() {
        min_value as i32
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_number_of_balloons("nlaebolko".to_string()), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_number_of_balloons("leetcode".to_string()), 0);
    }
}
