use super::factors::prime_factors;

pub fn phi(n: u64) -> u64 {
    let factors = prime_factors(n);
    let mut result = n;

    for p in factors {
        result = result * (p - 1) / p;
    }

    result
}
