pub fn gray_code(n: i32) -> Vec<i32> {
    let mut code: Vec<i32> = vec![0; 2i32.pow(n as u32) as usize];

    for i in 0..code.len() {
        code[i] = (i ^ (i >> 1)) as i32;
    }

    code
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(gray_code(2), [0, 1, 3, 2]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(gray_code(1), [0, 1]);
    }
}
