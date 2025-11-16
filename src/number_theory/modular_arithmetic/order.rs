/// Computes the multiplicative order of `n` modulo prime `p`.
///
/// Algorithm:
/// 1. Let m = p - 1 (because the group (Z/pZ)* has size p-1).
/// 2. Factorize m.
/// 3. For each prime factor q of m:
///    - If n^(m / q) ≡ 1 (mod p), then the order divides m / q.
///      Continue dividing out q as long as the congruence remains 1.
/// 4. The smallest divisor d of m for which n^d ≡ 1 (mod p) is the order.
/// 5. If the order is p - 1, then n is a primitive root modulo p.

// pub fn order(n: )