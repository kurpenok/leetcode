pub fn reverse_only_letters(s: String) -> String {
    let ascii_symbols = s
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    let mut reversed_s = String::new();
    let mut i = ascii_symbols.len();
    for symbol in s.chars() {
        if symbol.is_alphabetic() {
            reversed_s.push(ascii_symbols[i - 1]);
            i -= 1;
        } else {
            reversed_s.push(symbol);
        }
    }

    reversed_s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(reverse_only_letters("ab-cd".to_string()), "dc-ba");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba"
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!"
        );
    }
}
