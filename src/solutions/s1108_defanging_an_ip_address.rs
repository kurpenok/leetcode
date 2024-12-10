pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(defang_i_paddr("1.1.1.1".to_string()), "1[.]1[.]1[.]1");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            defang_i_paddr("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0"
        );
    }
}
