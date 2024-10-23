pub fn is_long_pressed_name(name: String, typed: String) -> bool {
    let name_symbols = name.chars().collect::<Vec<char>>();

    let mut i = 0;
    for t in typed.chars() {
        if i < name.len() && t == name_symbols[i] {
            i += 1;
        } else if t != name_symbols[i.saturating_sub(1)] {
            return false;
        }
    }

    i == name.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_long_pressed_name("alex".to_string(), "aalex".to_string()),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()),
            false
        );
    }
}
