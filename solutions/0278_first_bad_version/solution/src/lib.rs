fn is_bad_version(version: i32) -> bool {
    true
}

pub fn first_bad_version(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;

    while left < right {
        let middle = left + ((right - left) / 2);
        if is_bad_version(middle) {
            right = middle;
        } else {
            left = middle + 1;
        }
    }

    left
}
