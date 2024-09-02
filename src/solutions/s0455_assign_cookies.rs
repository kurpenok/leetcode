pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut g = g;
    g.sort();

    let mut s = s;
    s.sort();

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut result: i32 = 0;

    while j < s.len() && i < g.len() {
        if s[j] >= g[i] {
            result += 1;
            i += 1;
        }
        j += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
    }
}
