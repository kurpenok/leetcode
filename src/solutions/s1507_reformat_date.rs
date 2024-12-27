use std::collections::HashMap;

pub fn reformat_date(date: String) -> String {
    let months = HashMap::from([
        ("Jan", "1"),
        ("Feb", "2"),
        ("Mar", "3"),
        ("Apr", "4"),
        ("May", "5"),
        ("Jun", "6"),
        ("Jul", "7"),
        ("Aug", "8"),
        ("Sep", "9"),
        ("Oct", "10"),
        ("Nov", "11"),
        ("Dec", "12"),
    ]);

    let index = match (
        date.find("st"),
        date.find("nd"),
        date.find("rd"),
        date.find("th"),
    ) {
        (Some(index), None, None, None) => index,
        (None, Some(index), None, None) => index,
        (None, None, Some(index), None) => index,
        (None, None, None, Some(index)) => index,
        _ => 0,
    };

    let day = date[..index].parse::<usize>().unwrap();
    let month = months[&date[index + 3..index + 6]]
        .parse::<usize>()
        .unwrap();
    let year = &date[date.len() - 4..];

    format!("{}-{:02}-{:02}", year, month, day)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(reformat_date("20th Oct 2052".to_string()), "2052-10-20");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(reformat_date("6th Jun 1933".to_string()), "1933-06-06");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(reformat_date("26th May 1960".to_string()), "1960-05-26");
    }
}
