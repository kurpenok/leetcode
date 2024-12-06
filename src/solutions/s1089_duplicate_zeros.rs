pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut new_arr = Vec::new();

    for i in 0..arr.len() {
        if new_arr.len() >= arr.len() {
            break;
        }

        if arr[i] == 0 {
            new_arr.push(0);
        }
        new_arr.push(arr[i]);
    }
    new_arr = new_arr[..arr.len()].to_vec();

    arr.clear();
    arr.extend(new_arr);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut arr);

        assert_eq!(arr, [1, 0, 0, 2, 3, 0, 0, 4]);
    }

    #[test]
    fn test_case_2() {
        let mut arr = vec![1, 2, 3];
        duplicate_zeros(&mut arr);

        assert_eq!(arr, [1, 2, 3]);
    }
}
