use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit: i32 = 0;
        let mut local_min: i32 = prices[0];

        for i in 0..prices.len()-1 {
            local_min = min(local_min, prices[i]);
            profit = max(prices[i + 1] - local_min, profit);
        }

        return  profit;
    }
}
