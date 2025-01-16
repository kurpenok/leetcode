pub fn greatest_letter(s: String) -> String {
    let mut symbols = [false; 58];
    s.bytes().for_each(|b| symbols[(b - b'A') as usize] = true);

    match symbols[..26]
        .iter()
        .enumerate()
        .filter(|&(i, &value)| value && symbols[i + 32])
        .max()
    {
        Some((i, _)) => ((i as u8 + b'A') as char).to_string(),
        None => String::new(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(greatest_letter("lEeTcOdE".to_string()), "E");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(greatest_letter("arRAzFif".to_string()), "R");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(greatest_letter("AbCdEfGhIjK".to_string()), "");
    }
}
