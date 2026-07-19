pub fn capitalize_title(title: String) -> String {
    title
        .split_whitespace()
        .map(|w| {
            if w.len() <= 2 {
                w.to_lowercase()
            } else {
                format!("{}{}", w[..1].to_uppercase(), w[1..].to_lowercase())
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
