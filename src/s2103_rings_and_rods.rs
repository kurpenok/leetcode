pub fn count_points(rings: String) -> i32 {
    let mut rods = [0u8; 10];

    for ring in rings.as_bytes().chunks(2) {
        let color = ring[0];
        let index = (ring[1] - b'0') as usize;
        match color {
            b'R' => rods[index] |= 1,
            b'G' => rods[index] |= 2,
            b'B' => rods[index] |= 4,
            _ => unreachable!(),
        }
    }

    rods.iter().filter(|&&rod| rod == 7).count() as i32
}
