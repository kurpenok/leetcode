pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    ((m * n) == original.len() as i32)
        .then(|| {
            original
                .chunks_exact(n as usize)
                .map(|chunk| chunk.to_vec())
                .collect()
        })
        .unwrap_or_default()
}
