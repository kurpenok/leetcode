use std::collections::HashSet;

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut unique_emails: HashSet<String> = HashSet::new();

    for email in &emails {
        let name = &email[..email.find('@').unwrap()];
        let domain = &email[email.find('@').unwrap() + 1..];

        let new_name = name.chars().filter(|&c| c != '.').collect::<String>();
        let new_name = match new_name.find('+') {
            Some(plus_index) => &new_name[..plus_index].to_string(),
            None => &new_name.to_string(),
        };

        unique_emails.insert(format!("{}@{}", new_name, domain));
    }

    unique_emails.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string()
            ]),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            num_unique_emails(vec![
                "a@leetcode.com".to_string(),
                "b@leetcode.com".to_string(),
                "c@leetcode.com".to_string()
            ]),
            3
        );
    }
}
