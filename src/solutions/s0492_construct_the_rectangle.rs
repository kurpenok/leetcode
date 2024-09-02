pub fn construct_rectangle(area: i32) -> Vec<i32> {
    let mut root: i32 = (area as f64).sqrt() as i32 + 1;
    while area % root != 0 {
        root -= 1;
    }

    if area / root < root {
        return vec![root, area / root];
    }
    vec![area / root, root]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(construct_rectangle(37), vec![37, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(construct_rectangle(122122), vec![427, 286]);
    }
}
