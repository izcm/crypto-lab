use crate::number_theory::factors::gcd::extended_gcd;
use crate::number_theory::factors::is_prime::is_prime;
use crate::number_theory::factors::mod_power::mod_pow;

pub fn inverse(a: u64, m: u64) -> Option<u64> {
    if is_prime(m) {
        Some(mod_pow(m, a - 2, m - 2))
    } else {
        inverse_by_extended_euclidean(a, m)
    }
}

pub fn inverse_by_extended_euclidean(a: u64, m: u64) -> Option<u64> {
    let (g, u, _) = extended_gcd(a as i64, m as i64);

    if g != 1 {
        return None; // inverse does NOT exist
    }

    let m_i = m as i64;

    // normalize negative u
    let inv_i = ((u % m_i) + m_i) % m_i;

    Some(inv_i as u64)
}
