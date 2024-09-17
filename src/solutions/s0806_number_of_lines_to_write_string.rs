pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut lines_count = 1;
    let mut line_width = 0;

    for c in s.chars() {
        let symbol_width = widths[c as usize - 'a' as usize];

        if line_width + symbol_width > 100 {
            lines_count += 1;
            line_width = 0;
        }

        line_width += symbol_width;
    }

    vec![lines_count, line_width]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            ),
            [3, 60]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            ),
            [2, 4]
        );
    }
}
