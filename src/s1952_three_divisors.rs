pub fn is_three(n: i32) -> bool {
    n.isqrt() * n.isqrt() == n && (2..=n.isqrt()).filter(|&m| n % m == 0).count() == 1
}
