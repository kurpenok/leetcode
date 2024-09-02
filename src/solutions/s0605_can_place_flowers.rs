use std::collections::VecDeque;

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut temp_flowerbed: VecDeque<i32> = VecDeque::from(flowerbed);
    temp_flowerbed.push_front(0);
    temp_flowerbed.push_back(0);

    let mut flowerbed: Vec<i32> = Vec::from(temp_flowerbed);
    let mut n = n;

    for i in 1..flowerbed.len() - 1 {
        if flowerbed[i] == 0 && flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
            flowerbed[i] = 1;
            n -= 1;
        }
    }

    if n < 1 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1, 0, 0], 2), true);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(can_place_flowers(vec![0], 1), true);
    }
}
