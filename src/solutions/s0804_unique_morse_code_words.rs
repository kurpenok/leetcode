use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let mut words_codes: HashSet<String> = HashSet::new();
    let morse_code: Vec<&str> = vec![
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    for word in words {
        let mut word_code = String::new();
        for char in word.chars() {
            let char_code = (char as u32 - 97) as usize;
            word_code.push_str(morse_code[char_code]);
        }
        words_codes.insert(word_code);
    }

    words_codes.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ]),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(unique_morse_representations(vec!["a".to_string()]), 1);
    }
}
