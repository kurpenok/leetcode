pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut num_bottles = num_bottles;
    let mut count: i32 = 0;

    while num_bottles >= num_exchange {
        let sets_drank = num_bottles / num_exchange;
        count += sets_drank * num_exchange;
        num_bottles = num_bottles - (sets_drank * num_exchange) + sets_drank;
    }

    count + num_bottles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(num_water_bottles(9, 3), 13);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(num_water_bottles(15, 4), 19);
    }
}
