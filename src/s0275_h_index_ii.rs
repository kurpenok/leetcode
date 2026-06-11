pub fn h_index(citations: Vec<i32>) -> i32 {
    if citations.len() == 0 {
        return 0;
    }

    let mut low = 0;
    let mut high = citations.len();
    while low < high {
        let middle = (low + high) / 2;
        if citations[middle] >= (citations.len() - middle) as i32 {
            high = middle;
        } else {
            low = middle + 1;
        }
    }

    (citations.len() - low) as i32
}
