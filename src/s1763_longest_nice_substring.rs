fn is_nice(sub: &str) -> bool {
    let mut has_lower = [0; 26];
    let mut has_upper = [0; 26];

    for c in sub.chars() {
        if c.is_ascii_lowercase() {
            has_lower[(c as u8 - b'a') as usize] = 1;
        } else if c.is_ascii_uppercase() {
            has_upper[(c as u8 - b'A') as usize] = 1;
        }
    }

    has_lower == has_upper
}

pub fn longest_nice_substring(s: String) -> String {
    let mut best_start = 0;
    let mut best_end = 0;

    for i in 0..s.len() {
        for j in i..s.len() {
            if is_nice(&s[i..=j]) && j - i > best_end - best_start {
                best_start = i;
                best_end = j;
            }
        }
    }

    if best_start == best_end {
        "".to_string()
    } else {
        s[best_start..=best_end].to_string()
    }
}
