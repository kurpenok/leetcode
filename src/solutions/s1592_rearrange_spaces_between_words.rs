pub fn reorder_spaces(text: String) -> String {
    let count_whitespace: usize = text.chars().filter(|x| *x == ' ').count();
    if count_whitespace == 0 {
        return text;
    }

    let div_mod = |x, y| (x / y, x % y);
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.len() == 1 {
        words[0].to_owned() + " ".repeat(count_whitespace).as_str()
    } else {
        let (div_sep, mod_sep) = div_mod(count_whitespace, words.len() - 1);
        words.join(" ".repeat(div_sep).as_str()) + " ".repeat(mod_sep).as_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            reorder_spaces("  this   is  a sentence ".to_string()),
            "this   is   a   sentence"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            reorder_spaces(" practice   makes   perfect".to_string()),
            "practice   makes   perfect "
        );
    }
}
