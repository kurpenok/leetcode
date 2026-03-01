use leetcode::s0200_number_of_islands::num_islands;

#[test]
fn test_case_1() {
    assert_eq!(
        num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );
}

#[test]
fn test_case_2() {
    assert_eq!(
        num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ]),
        3
    );
}
