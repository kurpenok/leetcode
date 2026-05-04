use std::collections::HashMap;

pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    let pieces_every_first: HashMap<i32, Vec<i32>> =
        pieces.into_iter().map(|piece| (piece[0], piece)).collect();

    let mut i = 0;
    while i < arr.len() {
        let piece = match pieces_every_first.get(&arr[i]) {
            Some(piece) => piece,
            None => return false,
        };
        for &p in piece {
            if arr.get(i) != Some(&p) {
                return false;
            }
            i += 1;
        }
    }

    true
}
