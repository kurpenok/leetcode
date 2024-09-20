pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let mut shortest_distances_to_char: Vec<i32> = Vec::new();

    let mut char_indexes: Vec<i32> = Vec::new();
    s.chars().enumerate().for_each(|(i, symbol)| {
        if symbol == c {
            char_indexes.push(i as i32);
        }
    });

    for i in 0..s.len() {
        let mut distance: i32 = i32::MAX;

        for index in &char_indexes {
            distance = std::cmp::min(distance, (i as i32 - index).abs());
        }

        shortest_distances_to_char.push(distance);
    }

    shortest_distances_to_char
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            shortest_to_char("loveleetcode".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(shortest_to_char("aaab".to_string(), 'b'), vec![3, 2, 1, 0]);
    }
}
