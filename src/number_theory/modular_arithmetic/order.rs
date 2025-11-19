use crate::number_theory::factors::gcd::gcd;
use crate::number_theory::factors::is_prime::is_prime;
use crate::number_theory::factors::mod_power::mod_pow;

/// Computes the multiplicative order of `a` modulo `m`.
///
/// The order is the smallest `k` such that a^k ≡ 1 (mod m).
/// Requires gcd(a, m) = 1.
///
/// Steps:
/// - let group_size = (m - 1) if m is prime, else φ(m)
/// - factor group_size
/// - start with k = group_size
/// - for each prime factor q: while a^(d/q) ≡ 1 (mod m), set d = d / q
/// - return k
///
/// note: if order = group_size => `a` is a primitive root mod m

pub fn order(a: u64, m: u64) -> u64 {
    // order only exists if a is in the unit group mod m
    if gcd(a as i64, m as i64) != 1 {
        return 0; // a is not coprime
    }

    // 2. decide group size
    let mut g = if is_prime(m) {
        m - 1
    } else {
        // TODO: implement phi
        0
    };

    // 3. get prime factors of g
    let factors = prime_factors(g);

    // 4.
    for q in factors {
        while g % q == 0 {
            if mod_pow(a, g / q, m) == 1 {
                g /= q;
            } else {
                break;
            }
        }
    }

    g // in rust last expression in a function is auto returned if we don't put a semicolon
}

pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            factors.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}
