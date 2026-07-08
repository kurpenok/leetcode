fn is_valid(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();

    let mut hyphen_count = 0;
    let mut punct_count = 0;

    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            return false;
        }

        if c == '-' {
            hyphen_count += 1;
            if hyphen_count > 1 {
                return false;
            }

            if i == 0 || i == chars.len() - 1 {
                return false;
            }

            if !chars[i - 1].is_ascii_alphabetic() || !chars[i + 1].is_ascii_alphabetic() {
                return false;
            }
        }

        if c == '!' || c == '.' || c == ',' {
            punct_count += 1;
            if punct_count > 1 {
                return false;
            }

            if i != chars.len() - 1 {
                return false;
            }
        }
    }

    true
}

pub fn count_valid_words(sentence: String) -> i32 {
    sentence
        .split_whitespace()
        .filter(|word| is_valid(word))
        .count() as i32
}
