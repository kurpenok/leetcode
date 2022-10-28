impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut symbols: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let mut title: String = column_title.chars().rev().collect();
        let mut number: i32 = 0;
        let mut index: usize = 0;

        for (i, title_symbol) in title.chars().enumerate() {
            for j in 0..symbols.len() {
                if (title_symbol == symbols[j]) {
                    index = j;
                    break;
                }
            }
            number += ((index + 1) as i32) * 26i32.pow((i as u32));
        }

        number
    }
}
