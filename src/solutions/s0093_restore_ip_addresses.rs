fn ok(seg: &str) -> bool {
    seg.len() > 0
        && seg.len() <= 3
        && !(seg.starts_with('0') && seg.len() > 1)
        && seg.parse::<u32>().unwrap() <= 255
}

pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut ip_addresses = vec![];
    let n = s.len();

    for i in 1..4 {
        for j in i + 1..i + 4 {
            for k in j + 1..j + 4 {
                if k < n {
                    let seg1 = &s[..i];
                    let seg2 = &s[i..j];
                    let seg3 = &s[j..k];
                    let seg4 = &s[k..];

                    if ok(seg1) && ok(seg2) && ok(seg3) && ok(seg4) {
                        ip_addresses.push(format!("{}.{}.{}.{}", seg1, seg2, seg3, seg4));
                    }
                }
            }
        }
    }

    ip_addresses
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            restore_ip_addresses("25525511135".to_string()),
            ["255.255.11.135", "255.255.111.35"]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(restore_ip_addresses("0000".to_string()), ["0.0.0.0"]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            restore_ip_addresses("101023".to_string()),
            [
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }
}
