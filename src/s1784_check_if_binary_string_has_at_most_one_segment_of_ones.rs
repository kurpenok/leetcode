pub fn check_ones_segment(s: String) -> bool {
    let mut zeros_segment_start_flag = false;
    let mut zeros_segment_end_flag = false;

    for c in s.chars() {
        if c == '0' && zeros_segment_end_flag {
            return false;
        } else if c == '0' {
            zeros_segment_start_flag = true;
        } else if c == '1' && zeros_segment_start_flag {
            zeros_segment_end_flag = true;
        }
    }

    if zeros_segment_end_flag { false } else { true }
}
