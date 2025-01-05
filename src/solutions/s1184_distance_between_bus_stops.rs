pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let (s0, s1) = match start <= destination {
        true => (start as usize, destination as usize),
        false => (destination as usize, start as usize),
    };

    distance[s0..s1]
        .iter()
        .sum::<i32>()
        .min(distance[..s0].iter().sum::<i32>() + distance[s1..].iter().sum::<i32>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3), 4);
    }
}
