pub fn min_operations(s: String) -> i32 {
    let mut operations_1 = 0;
    let mut operations_2 = 0;

    for (i, c) in s.char_indices() {
        let expected_1 = if i % 2 == 0 { '0' } else { '1' };
        let expected_2 = if i % 2 == 0 { '1' } else { '0' };

        if c != expected_1 {
            operations_1 += 1;
        }
        if c != expected_2 {
            operations_2 += 1;
        }
    }

    operations_1.min(operations_2)
}
