pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn is_safe(vis: &Vec<Vec<bool>>, i: isize, j: isize, m: usize, n: usize) -> bool {
        i >= 0 && j >= 0 && i < m as isize && j < n as isize && !vis[i as usize][j as usize]
    }

    fn rec(
        i: isize,
        j: isize,
        index: usize,
        m: usize,
        n: usize,
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        vis: &mut Vec<Vec<bool>>,
    ) -> bool {
        if index == word.len() {
            return true;
        }
        if !is_safe(vis, i, j, m, n) || board[i as usize][j as usize] != word[index] {
            return false;
        }

        vis[i as usize][j as usize] = true;

        let left = rec(i - 1, j, index + 1, m, n, board, word, vis);
        let right = rec(i + 1, j, index + 1, m, n, board, word, vis);
        let up = rec(i, j - 1, index + 1, m, n, board, word, vis);
        let down = rec(i, j + 1, index + 1, m, n, board, word, vis);

        vis[i as usize][j as usize] = false;

        left || right || up || down
    }

    let m = board.len();
    let n = board[0].len();
    let word_chars: Vec<char> = word.chars().collect();

    for i in 0..m {
        for j in 0..n {
            if board[i][j] == word_chars[0] {
                let mut vis = vec![vec![false; n]; m];
                if rec(
                    i as isize,
                    j as isize,
                    0,
                    m,
                    n,
                    &board,
                    &word_chars,
                    &mut vis,
                ) {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_string()
            ),
            true
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            ),
            false
        );
    }
}
