pub fn calculate(s: String) -> i32 {
    let mut stack = Vec::new();

    let s_chars = s.chars().collect::<Vec<char>>();
    let n = s_chars.len();

    let mut number = 0;
    let mut operation = '+';

    for i in 0..n {
        let c = s_chars[i];

        if c.is_ascii_digit() {
            number = number * 10 + (c as u8 - b'0') as i32;
        }

        if (!c.is_ascii_digit() && c != ' ') || i == n - 1 {
            match operation {
                '+' => stack.push(number),
                '-' => stack.push(-number),
                '*' => {
                    let last = stack.pop().unwrap();
                    stack.push(last * number);
                }
                '/' => {
                    let last = stack.pop().unwrap();
                    stack.push(last / number);
                }
                _ => unreachable!(),
            }

            operation = c;
            number = 0;
        }
    }

    stack.iter().sum()
}
