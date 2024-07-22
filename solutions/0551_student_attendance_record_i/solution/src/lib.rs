pub fn check_record(s: String) -> bool {
    let mut absence: usize = 0;
    let mut delays: usize = 0;

    for c in s.chars() {
        if c == 'A' {
            absence += 1;
            delays = 0;
            if absence > 1 {
                return false;
            }
        } else if c == 'L' {
            delays += 1;
            if delays > 2 {
                return false;
            }
        } else if c == 'P' {
            delays = 0;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(check_record("PPALLP".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(check_record("PPALLL".to_string()), false);
    }
}
