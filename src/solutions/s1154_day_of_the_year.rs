pub fn day_of_year(date: String) -> i32 {
    let months: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let year = date[..4].parse::<i32>().unwrap();
    let month = date[5..7].parse::<usize>().unwrap() - 1;
    let days = date[8..].parse::<i32>().unwrap();

    let is_leap_year = month >= 2 && ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0);

    months[..month].iter().sum::<i32>() + days + is_leap_year as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(day_of_year("2019-01-09".to_string()), 9);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(day_of_year("2019-02-10".to_string()), 41);
    }
}
