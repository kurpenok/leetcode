pub fn sort_sentence(s: String) -> String {
    let mut words: Vec<(usize, String)> = s
        .split_whitespace()
        .map(|w| {
            let position = w.chars().last().unwrap().to_digit(10).unwrap() as usize;
            let word = &w[..w.len() - 1];
            (position, word.to_string())
        })
        .collect();

    words.sort_by_key(|&(position, _)| position);
    words
        .into_iter()
        .map(|(_, word)| word)
        .collect::<Vec<String>>()
        .join(" ")
}
