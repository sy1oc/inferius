pub mod obj;
pub fn binom(n: i32, k: i32) -> u64 {
    if k > n {
        return 0u64;
    }
    if k == 0 || k == n {
        return 1u64;
    }
    if k > n - k {
        return binom(n, n - k);
    }
    let mut result = 0u64;
    let mut i = 0u64;
    while i <= k as u64 {
        result = result
            .checked_mul((n as u64 - k as u64 + i))
            .unwrap_or(u64::MAX)
            / (i + 1u64);
        i += 1;
    }
    result
}
