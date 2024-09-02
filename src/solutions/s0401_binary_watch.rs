pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let mut result = Vec::new();

    for h in 0i32..12 {
        for m in 0i32..60 {
            if h.count_ones() + m.count_ones() == turned_on as u32 {
                result.push(format!("{h}:{m:0>2}"));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            read_binary_watch(1),
            vec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
        );
    }

    #[test]
    fn test_case_2() {
        let time: Vec<String> = Vec::new();
        assert_eq!(read_binary_watch(9), time);
    }
}
