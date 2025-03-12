pub fn sort_string(s: String) -> String {
    let mut sorted_s = String::new();
    let mut counter = [0; 26];

    let mut increment: i32 = 1;
    let mut index: i32 = 0;

    s.as_bytes()
        .iter()
        .for_each(|b| counter[(*b - b'a') as usize] += 1);

    while sorted_s.len() < s.len() {
        if counter[index as usize] > 0 {
            sorted_s.push((b'a' + index as u8) as char);
            counter[index as usize] -= 1;
        };

        index += increment;
        if index < 0 || index > 25 {
            index = if index < 0 { 0 } else { 25 };
            increment *= -1;
        };
    }

    sorted_s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(sort_string("aaaabbbbcccc".to_string()), "abccbaabccba");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sort_string("rat".to_string()), "art");
    }
}
