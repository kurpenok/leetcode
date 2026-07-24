pub fn count_operations(num1: i32, num2: i32) -> i32 {
    let mut num1 = num1;
    let mut num2 = num2;
    let mut operations_counter = 0;

    while num1 != 0 && num2 != 0 {
        if num1 >= num2 {
            operations_counter += num1 / num2;
            num1 %= num2;
        } else {
            operations_counter += num2 / num1;
            num2 %= num1;
        }
    }

    operations_counter
}
