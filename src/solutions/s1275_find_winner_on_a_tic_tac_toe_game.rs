pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let mut tics: Vec<Vec<i32>> = Vec::new();
    let mut tacs: Vec<Vec<i32>> = Vec::new();

    for i in 0..moves.len() {
        if i % 2 == 0 {
            tics.push(moves[i].clone());
        } else {
            tacs.push(moves[i].clone());
        }
    }

    if (tics.contains(&vec![0, 0]) && tics.contains(&vec![0, 1]) && tics.contains(&vec![0, 2]))
        || (tics.contains(&vec![1, 0]) && tics.contains(&vec![1, 1]) && tics.contains(&vec![1, 2]))
        || (tics.contains(&vec![2, 0]) && tics.contains(&vec![2, 1]) && tics.contains(&vec![2, 2]))
        || (tics.contains(&vec![0, 0]) && tics.contains(&vec![1, 0]) && tics.contains(&vec![2, 0]))
        || (tics.contains(&vec![0, 1]) && tics.contains(&vec![1, 1]) && tics.contains(&vec![2, 1]))
        || (tics.contains(&vec![0, 2]) && tics.contains(&vec![1, 2]) && tics.contains(&vec![2, 2]))
        || (tics.contains(&vec![0, 0]) && tics.contains(&vec![1, 1]) && tics.contains(&vec![2, 2]))
        || (tics.contains(&vec![0, 2]) && tics.contains(&vec![1, 1]) && tics.contains(&vec![2, 0]))
    {
        return "A".to_string();
    }

    if (tacs.contains(&vec![0, 0]) && tacs.contains(&vec![0, 1]) && tacs.contains(&vec![0, 2]))
        || (tacs.contains(&vec![1, 0]) && tacs.contains(&vec![1, 1]) && tacs.contains(&vec![1, 2]))
        || (tacs.contains(&vec![2, 0]) && tacs.contains(&vec![2, 1]) && tacs.contains(&vec![2, 2]))
        || (tacs.contains(&vec![0, 0]) && tacs.contains(&vec![1, 0]) && tacs.contains(&vec![2, 0]))
        || (tacs.contains(&vec![0, 1]) && tacs.contains(&vec![1, 1]) && tacs.contains(&vec![2, 1]))
        || (tacs.contains(&vec![0, 2]) && tacs.contains(&vec![1, 2]) && tacs.contains(&vec![2, 2]))
        || (tacs.contains(&vec![0, 0]) && tacs.contains(&vec![1, 1]) && tacs.contains(&vec![2, 2]))
        || (tacs.contains(&vec![0, 2]) && tacs.contains(&vec![1, 1]) && tacs.contains(&vec![2, 0]))
    {
        return "B".to_string();
    }

    if moves.len() == 9 {
        "Draw".to_string()
    } else {
        "Pending".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            tictactoe(vec![
                vec![0, 0],
                vec![2, 0],
                vec![1, 1],
                vec![2, 1],
                vec![2, 2]
            ]),
            "A"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![2, 0]
            ]),
            "B"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2]
            ]),
            "Draw"
        );
    }
}
