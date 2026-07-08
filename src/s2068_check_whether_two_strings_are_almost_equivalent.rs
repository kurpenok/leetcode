pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let mut freqs = vec![0i32; 26];
    word1.bytes().for_each(|b| freqs[(b - b'a') as usize] += 1);
    word2.bytes().for_each(|b| freqs[(b - b'a') as usize] -= 1);
    freqs.iter().all(|f| f.abs() <= 3)
}
