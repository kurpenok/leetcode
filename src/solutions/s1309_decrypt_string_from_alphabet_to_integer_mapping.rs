use std::collections::HashMap;

pub fn freq_alphabets(s: String) -> String {
    let mut decrypted_s: String = String::new();

    let alphabet: HashMap<&str, char> = HashMap::from_iter(vec![
        ("1", 'a'),
        ("2", 'b'),
        ("3", 'c'),
        ("4", 'd'),
        ("5", 'e'),
        ("6", 'f'),
        ("7", 'g'),
        ("8", 'h'),
        ("9", 'i'),
        ("10#", 'j'),
        ("11#", 'k'),
        ("12#", 'l'),
        ("13#", 'm'),
        ("14#", 'n'),
        ("15#", 'o'),
        ("16#", 'p'),
        ("17#", 'q'),
        ("18#", 'r'),
        ("19#", 's'),
        ("20#", 't'),
        ("21#", 'u'),
        ("22#", 'v'),
        ("23#", 'w'),
        ("24#", 'x'),
        ("25#", 'y'),
        ("26#", 'z'),
    ]);

    let s_chars: Vec<char> = s.chars().collect::<Vec<char>>();
    let mut i: usize = 0;

    while i < s_chars.len() {
        if i + 2 < s_chars.len() && s_chars[i + 2] == '#' {
            decrypted_s.push(alphabet[&s[i..i + 3]]);
            i += 3;
        } else {
            decrypted_s.push(alphabet[&s[i..i + 1]]);
            i += 1;
        }
    }

    decrypted_s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(freq_alphabets("10#11#12".to_string()), "jkab");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(freq_alphabets("1326#".to_string()), "acz");
    }
}
