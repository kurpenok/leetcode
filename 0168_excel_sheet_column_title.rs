impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut symbols: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let mut title: String = "".to_string();
        let mut column: i32 = column_number;
        let mut index: usize;

        while column != 0 {
            if column % 26 == 0 {
                index = 25;
            } else {
                index = ((column % 26) - 1) as usize;
            }
            title.push(symbols[index]);
            print!("{}", index);

            column -= 1;
            column /= 26;
        }
        println!();

        return title.chars().rev().collect();
    }
}
