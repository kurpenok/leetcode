pub fn cells_in_range(s: String) -> Vec<String> {
    let bytes = s.as_bytes();
    let (col_1, row_1) = (bytes[0] as char, bytes[1] as char);
    let (col_2, row_2) = (bytes[3] as char, bytes[4] as char);

    (col_1..=col_2)
        .flat_map(|c| (row_1..=row_2).map(move |r| format!("{}{}", c, r)))
        .collect()
}
