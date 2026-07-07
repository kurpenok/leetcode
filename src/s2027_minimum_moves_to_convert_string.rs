pub fn minimum_moves(s: String) -> i32 {
    let mut s_chars = s.chars();
    let mut moves = 0;

    while let Some(ch) = s_chars.next() {
        if ch == 'X' {
            moves += 1;
            s_chars.nth(1);
        }
    }

    moves
}
