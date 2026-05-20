pub fn maximum_time(time: String) -> String {
    let mut result_time = vec![0u32; 4];
    let time_chars: Vec<char> = time.chars().collect();

    if time_chars[0] == '?' {
        result_time[0] = if "?0123".contains(time_chars[1]) {
            2
        } else {
            1
        };
    } else {
        result_time[0] = time_chars[0].to_digit(10).unwrap();
    }

    if time_chars[1] == '?' {
        result_time[1] = if result_time[0] == 2 { 3 } else { 9 };
    } else {
        result_time[1] = time_chars[1].to_digit(10).unwrap();
    }

    result_time[2] = if time_chars[3] == '?' {
        5
    } else {
        time_chars[3].to_digit(10).unwrap()
    };
    result_time[3] = if time_chars[4] == '?' {
        9
    } else {
        time_chars[4].to_digit(10).unwrap()
    };

    format!(
        "{}{}:{}{}",
        result_time[0], result_time[1], result_time[2], result_time[3]
    )
}
