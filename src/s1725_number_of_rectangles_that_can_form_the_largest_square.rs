pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let mut max_len = 0;
    let mut count = 0;

    for rect in rectangles {
        let side = rect[0].min(rect[1]);
        if side > max_len {
            max_len = side;
            count = 1;
        } else if side == max_len {
            count += 1;
        }
    }

    count
}
