pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let mut candies = candies as usize;
    let num_people = num_people as usize;
    let mut peoples: Vec<i32> = vec![0; num_people];

    let mut i = 0;
    loop {
        let required_candies = i + 1;

        if candies < required_candies {
            peoples[i % num_people] += candies as i32;
            break;
        }

        peoples[i % num_people] += required_candies as i32;
        candies -= required_candies;
        i += 1;
    }

    peoples
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(distribute_candies(7, 4), [1, 2, 3, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(distribute_candies(10, 3), [5, 2, 3]);
    }
}
