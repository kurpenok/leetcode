pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
        return s;
    }

    let s_chars: Vec<char> = s.chars().collect();
    let num_rows = num_rows as usize;

    let mut zigzag: Vec<Vec<char>> = vec![vec![' '; s.len()]; num_rows as usize];
    let mut index = 0;
    let mut cycle = 0;
    let mut row = 0;
    let mut column = 0;

    while index < s.len() {
        zigzag[row][column] = s_chars[index];

        println!("{} {} {} {}", index, cycle, row, column);

        if cycle % 2 == 0 {
            row += 1;
        } else {
            column += 1;
            row -= 1;
        }

        if row == 0 || row == num_rows - 1 {
            cycle += 1;
        }

        index += 1;
    }

    let mut zigzag_s = String::new();
    for row in zigzag {
        for symbol in row {
            if symbol != ' ' {
                zigzag_s.push(symbol);
            }
        }
    }

    zigzag_s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(convert("A".to_string(), 1), "A");
    }
}
