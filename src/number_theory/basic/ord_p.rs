/// Computes the order (number of occurances) of prime p in the factorization of a number n.
pub fn ord_p(n: u64, p: u64) -> u32 {
    if n == 0 {
        return 0; // ord_p(0) undefined; handle however you want
    }

    if p == 2 {
        return ord2(n);
    }

    let mut tmp = n;
    let mut count = 0;

    while tmp % p == 0 {
        tmp /= p;
        count += 1;
    }

    count
}

/// Computes the orde order of the prime 2 in a given number n
fn ord2(n: u64) -> u32 {
    n.trailing_zeros()
}
