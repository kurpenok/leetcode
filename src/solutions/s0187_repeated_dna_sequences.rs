use std::collections::HashMap;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() < 10 {
        return vec![];
    }

    let mut result: Vec<String> = Vec::new();

    let mut counter: HashMap<&str, usize> = HashMap::new();
    for i in 0..s.len() - 9 {
        let t: &str = &s[i..i + 10];
        let count = counter.entry(t).or_insert(0);
        *count += 1;

        if *count == 2 {
            result.push(t.to_string());
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()),
            ["AAAAACCCCC", "CCCCCAAAAA"]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()),
            ["AAAAAAAAAA"]
        );
    }
}
