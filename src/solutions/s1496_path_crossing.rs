use std::collections::HashSet;

pub fn is_path_crossing(path: String) -> bool {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert((x, y));

    for c in path.chars() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'W' => x -= 1,
            'E' => x += 1,
            _ => (),
        }

        if set.contains(&(x, y)) {
            return true;
        }
        set.insert((x, y));
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_path_crossing("NES".to_string()), false);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_path_crossing("NESWW".to_string()), true);
    }
}
