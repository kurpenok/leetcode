fn check_connect(board: &mut Vec<Vec<char>>, mask: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    if !mask[i][j] && board[i][j] == 'O' {
        mask[i][j] = true;
    } else {
        return;
    }

    if i > 0 {
        check_connect(board, mask, i - 1, j);
    }

    if i < board.len() - 1 {
        check_connect(board, mask, i + 1, j);
    }

    if j > 0 {
        check_connect(board, mask, i, j - 1);
    }

    if j < board[0].len() - 1 {
        check_connect(board, mask, i, j + 1);
    }
}

pub fn solve(board: &mut Vec<Vec<char>>) {
    let mut mask = vec![vec![false; board[0].len()]; board.len()];

    for i in 0..board.len() {
        check_connect(board, &mut mask, i, 0);
        check_connect(board, &mut mask, i, board[0].len() - 1);
    }

    for j in 0..board[0].len() {
        check_connect(board, &mut mask, 0, j);
        check_connect(board, &mut mask, board.len() - 1, j);
    }

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if !mask[i][j] {
                board[i][j] = 'X';
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut board: Vec<Vec<char>> = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        solve(&mut board);

        assert_eq!(
            board,
            [
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'O', 'X', 'X']
            ]
        );
    }

    #[test]
    fn test_case_2() {
        let mut board: Vec<Vec<char>> = vec![vec!['X']];

        solve(&mut board);

        assert_eq!(board, [['X']]);
    }
}
