fn is_leap_year(year: usize) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn get_day_number_of_year(year: usize, month: usize, day: usize) -> usize {
    let months: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let is_leap_year = month > 1 && is_leap_year(year);
    months[..month].iter().sum::<usize>() + day + is_leap_year as usize
}

pub fn days_between_dates(date1: String, date2: String) -> i32 {
    let year_1 = date1[..4].parse::<usize>().unwrap();
    let month_1 = date1[5..7].parse::<usize>().unwrap() - 1;
    let day_1 = date1[8..].parse::<usize>().unwrap();
    let day_number_1 = get_day_number_of_year(year_1, month_1, day_1);

    let year_2 = date2[..4].parse::<usize>().unwrap();
    let month_2 = date2[5..7].parse::<usize>().unwrap() - 1;
    let day_2 = date2[8..].parse::<usize>().unwrap();
    let day_number_2 = get_day_number_of_year(year_2, month_2, day_2);

    if year_1 == year_2 {
        ((day_number_2 - day_number_1) as i32).abs()
    } else if year_1 < year_2 {
        let mut days_difference = if is_leap_year(year_1) {
            366 - day_number_1
        } else {
            365 - day_number_1
        };

        for year in year_1 + 1..year_2 {
            if is_leap_year(year) {
                days_difference += 366;
            } else {
                days_difference += 365;
            }
        }

        (days_difference + day_number_2) as i32
    } else {
        let mut days_difference = if is_leap_year(year_2) {
            366 - day_number_2
        } else {
            365 - day_number_2
        };

        for year in year_2 + 1..year_1 {
            if is_leap_year(year) {
                days_difference += 366;
            } else {
                days_difference += 365;
            }
        }

        (days_difference + day_number_1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()),
            1,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()),
            15,
        );
    }
}
