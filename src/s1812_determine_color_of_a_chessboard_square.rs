pub fn square_is_white(coordinates: String) -> bool {
    let symbol = coordinates.chars().nth(0).unwrap();
    let digit = coordinates.chars().nth(1).unwrap();

    if ("aceg".contains(symbol) && "1357".contains(digit))
        || ("bdfh".contains(symbol) && "2468".contains(digit))
    {
        false
    } else {
        true
    }
}
