pub fn to_hex(num: i32) -> String {
    let mut num = num as u32;

    let mut ans = Vec::with_capacity(8);
    let mask = 0xF;
    let map = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    loop {
        let idx = (num & mask) as usize;
        ans.push(map[idx]);
        num >>= 4;
        if num == 0 {
            break;
        }
    }

    ans.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(to_hex(26), "1a");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(to_hex(-1), "ffffffff");
    }
}
