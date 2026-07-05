pub fn reverse_prefix(word: String, ch: char) -> String {
    match word.find(ch) {
        Some(index) => {
            let reversed_prefix = &word[..=index].chars().rev().collect::<String>();
            let postfix = &word[index + 1..].to_string();
            format!("{}{}", reversed_prefix, postfix)
        }
        None => word,
    }
}
