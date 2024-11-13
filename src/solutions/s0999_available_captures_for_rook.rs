pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let mut attacked_pawns_count = 0;

    let mut r_position = (0, 0);
    'main: for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'R' {
                r_position = (i, j);
                break 'main;
            }
        }
    }

    for i in (0..r_position.0).rev() {
        if board[i][r_position.1] == 'B' {
            break;
        }
        if board[i][r_position.1] == 'p' {
            attacked_pawns_count += 1;
            break;
        }
    }

    for i in r_position.0 + 1..8 {
        if board[i][r_position.1] == 'B' {
            break;
        }
        if board[i][r_position.1] == 'p' {
            attacked_pawns_count += 1;
            break;
        }
    }

    for i in (0..r_position.1).rev() {
        if board[r_position.0][i] == 'B' {
            break;
        }
        if board[r_position.0][i] == 'p' {
            attacked_pawns_count += 1;
            break;
        }
    }

    for i in r_position.1 + 1..8 {
        if board[r_position.0][i] == 'B' {
            break;
        }
        if board[r_position.0][i] == 'p' {
            attacked_pawns_count += 1;
            break;
        }
    }

    attacked_pawns_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            ]),
            3,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            0,
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3,
        );
    }
}
