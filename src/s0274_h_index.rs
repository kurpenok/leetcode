pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut sorted_citations = citations;
    sorted_citations.sort();

    let mut h = 0;

    for i in (0..sorted_citations.len()).rev() {
        if sorted_citations[i] > h as i32 {
            h += 1;
        } else {
            break;
        }
    }

    h
}
