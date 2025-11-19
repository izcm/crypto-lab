use crate::number_theory::basic::is_prime;
use crate::number_theory::basic::phi;

use super::order::order;

// ❗ TODO: extend primitive root support to all m where (Z/mZ)* is cyclic:
//         m ∈ {2, 4, p^k, 2·p^k} for odd primes p.

/// Finds all primitive roots for prime p
pub fn primitive_roots(m: u64) -> Vec<u64> {
    if !has_primitive_roots(m) {
        return Vec::new();
    }

    let mut roots = Vec::new();

    for a in 1..m {
        if is_primitive_root(a, m) {
            roots.push(a);
        }
    }

    roots
}

fn has_primitive_roots(m: u64) -> bool {
    is_prime(m) // only accepting this for now
}

fn is_primitive_root(a: u64, m: u64) -> bool {
    // order(a, m) == (m - 1) // works for is_prime(m)
    order(a, m) == phi(m)
}
