pub fn to_goat_latin(sentence: String) -> String {
    let mut result = String::new();

    let words = sentence.split_whitespace().collect::<Vec<&str>>();
    words.iter().enumerate().for_each(|(i, word)| {
        let word_symbols = word.chars().collect::<Vec<char>>();
        let mut new_word = String::new();

        if "aeiouAEIOU".contains(word_symbols[0]) {
            new_word.push_str(&word);
        } else {
            new_word.push_str(&word[1..]);
            new_word.push(word_symbols[0]);
        }
        new_word.push_str("ma");

        new_word.push_str(&"a".repeat(i + 1));
        if i < words.len() - 1 {
            new_word.push(' ');
        }

        result.push_str(&new_word);
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            to_goat_latin("I speak Goat Latin".to_string()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()), "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa");
    }
}
