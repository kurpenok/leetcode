fn guess(num: i32) -> i32 {
    num
}

pub fn guess_number(n: i32) -> i32 {
    let mut left: i32 = 1;
    let mut right: i32 = n;

    while left < right {
        let middle = left + (right - left) / 2;
        let num = guess(middle);

        if num == 0 {
            return num;
        } else if num == -1 {
            right = middle - 1;
        } else if num == 1 {
            left = middle + 1;
        }
    }

    0
}
