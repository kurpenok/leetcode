pub fn largest_number(nums: Vec<i32>) -> String {
    let mut num_strs: Vec<String> = nums.into_iter().map(|num| num.to_string()).collect();
    num_strs.sort_by(|a, b| {
        let order1 = format!("{}{}", a, b);
        let order2 = format!("{}{}", b, a);
        order2.cmp(&order1)
    });

    let result = num_strs.join("");
    if result.starts_with('0') {
        "0".to_string()
    } else {
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(largest_number(vec![10, 2]), "210");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), "9534330");
    }
}
