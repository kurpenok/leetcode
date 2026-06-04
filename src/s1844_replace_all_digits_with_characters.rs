pub fn replace_digits(s: String) -> String {
    let mut bytes = s.into_bytes();

    for i in (1..bytes.len()).step_by(2) {
        bytes[i] = bytes[i - 1] + (bytes[i] - b'0');
    }

    String::from_utf8(bytes).unwrap()
}
