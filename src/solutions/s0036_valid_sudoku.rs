pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![0; 9];
    let mut columns = vec![0; 9];
    let mut boxes = vec![0; 9];

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                continue;
            }

            let num = board[i][j].to_digit(10).unwrap() - 1;
            let mask = 1 << num;
            let box_index = (i / 3) * 3 + j / 3;

            if (rows[i] & mask) != 0 || (columns[j] & mask) != 0 || (boxes[box_index] & mask) != 0 {
                return false;
            }

            rows[i] |= mask;
            columns[j] |= mask;
            boxes[box_index] |= mask;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }
}
