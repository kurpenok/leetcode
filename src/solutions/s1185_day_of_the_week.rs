pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
    let weekdays = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let is_leap_year = |x: i32| -> bool { x % 4 == 0 && (x % 100 != 0 || x % 400 == 0) };

    let num_days = (1971..year)
        .map(|y| 365 + is_leap_year(y) as i32)
        .sum::<i32>()
        + (0..month - 1)
            .map(|m| months[m as usize] + (m == 1 && is_leap_year(year)) as i32)
            .sum::<i32>()
        + day;

    let res_day = (4 + (num_days - 1)) % 7;
    weekdays[res_day as usize].to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(day_of_the_week(31, 8, 2019), "Saturday");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(day_of_the_week(18, 7, 1999), "Sunday");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(day_of_the_week(15, 8, 1993), "Sunday");
    }
}
