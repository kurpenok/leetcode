pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut products = vec![1; nums.len()];

    let mut prefix = 1;
    for i in 0..nums.len() {
        products[i] = prefix;
        prefix *= nums[i];
    }

    let mut suffix = 1;
    for i in (0..nums.len()).rev() {
        products[i] *= suffix;
        suffix *= nums[i];
    }

    products
}
