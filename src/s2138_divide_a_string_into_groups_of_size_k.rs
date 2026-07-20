pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let chars = s.chars().collect::<Vec<char>>();
    chars
        .chunks(k as usize)
        .map(|chunk| {
            chunk
                .iter()
                .copied()
                .chain(std::iter::repeat(fill))
                .take(k as usize)
                .collect()
        })
        .collect()
}
