pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut current_altitude = 0;
    let mut max_altitude = 0;

    for g in gain {
        current_altitude += g;
        max_altitude = max_altitude.max(current_altitude);
    }

    max_altitude
}
