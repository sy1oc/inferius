use crate::math::binom;

pub fn multi_geometric_expectation(n: i32, p: f64) -> f64 {
    let mut n_mod: i32 = n;
    let mut p_mod: f64 = p;
    if !(0f64 <= p && p <= 1f64) {
        p_mod = p.fract();
    }
    if !(0i32 <= n) {
        n_mod = n.abs();
    }
    let mut result = 0f64;
    for k in 1..=n_mod {
        result += binom(n, k) as f64 * (-1f64).powi(k) / ((1f64 - p_mod).powi(k) - 1f64);
    }
    result
}
