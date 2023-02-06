impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut base: f64 = x as f64;
        let mut power: f64 = n as f64;

        return base.powf(power);
    }
}
