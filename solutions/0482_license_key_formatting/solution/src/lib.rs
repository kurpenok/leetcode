pub fn license_key_formatting(s: String, k: i32) -> String {
    let s = s.replace("-", "").to_uppercase();
    let remain = s.len() % k as usize;

    let mut new_key = String::new();

    let mut blocks_count = s.len() / k as usize;
    if remain > 0 {
        blocks_count += 1;
    }

    for i in 0..blocks_count {
        if i == 0 && remain == 0 {
            new_key = format!("{}", &s[0..k as usize]);
            continue;
        } else if i == 0 && remain > 0 {
            new_key = format!("{}", &s[0..remain]);
            continue;
        }

        if remain > 0 {
            let left = (i - 1) * k as usize + remain;
            let right = i * k as usize + remain;
            new_key = format!("{}-{}", new_key, &s[left..right]);
        } else {
            let left = i * k as usize;
            let right = (i + 1) * k as usize;
            new_key = format!("{}-{}", new_key, &s[left..right]);
        }
    }

    new_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            license_key_formatting("5F3Z-2e-9-w".to_string(), 4),
            "5F3Z-2E9W"
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(license_key_formatting("2-5g-3-J".to_string(), 2), "2-5G-3J");
    }
}
