pub fn judge_circle(moves: String) -> bool {
    let mut coords: Vec<i32> = vec![0, 0];

    for m in moves.chars() {
        if m == 'U' {
            coords[1] += 1;
        } else if m == 'D' {
            coords[1] -= 1;
        } else if m == 'L' {
            coords[0] -= 1;
        } else if m == 'R' {
            coords[0] += 1;
        }
    }

    if coords[0] == 0 && coords[1] == 0 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(judge_circle("UD".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(judge_circle("LL".to_string()), false);
    }
}
