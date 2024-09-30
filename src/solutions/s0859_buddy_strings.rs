pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    if s == goal {
        let mut temp_s_symbols = s.chars().collect::<Vec<char>>();
        temp_s_symbols.sort();
        temp_s_symbols.dedup();
        if s.len() != temp_s_symbols.len() {
            return true;
        }
    }

    let s_symbols = s.chars().collect::<Vec<char>>();
    let goal_symbols = goal.chars().collect::<Vec<char>>();

    let mut diffs = Vec::new();
    for i in 0..s.len() {
        if s_symbols[i] != goal_symbols[i] {
            diffs.push((i, s_symbols[i]));
        }
    }

    if diffs.len() == 2 {
        let mut new_s_symbols = s.chars().collect::<Vec<char>>();
        new_s_symbols[diffs[0].0] = diffs[1].1;
        new_s_symbols[diffs[1].0] = diffs[0].1;
        return new_s_symbols.into_iter().collect::<String>() == goal;
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(buddy_strings("ab".to_string(), "ba".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(buddy_strings("ab".to_string(), "ab".to_string()), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(buddy_strings("aa".to_string(), "aa".to_string()), true);
    }
}
