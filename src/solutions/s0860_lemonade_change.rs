pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut my_bills = Vec::from([(5, 0), (10, 0)]);

    for bill in bills {
        if bill == 5 {
            my_bills[0].1 += 1;
        } else if bill == 10 && my_bills[0].1 > 0 {
            my_bills[0].1 -= 1;
            my_bills[1].1 += 1;
        } else if bill == 20 && my_bills[1].1 > 0 && my_bills[0].1 > 0 {
            my_bills[0].1 -= 1;
            my_bills[1].1 -= 1;
        } else if bill == 20 && my_bills[0].1 > 2 {
            my_bills[0].1 -= 3;
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(lemonade_change(vec![5, 5, 5, 10, 20]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(lemonade_change(vec![5, 5, 10, 10, 20]), false);
    }
}
