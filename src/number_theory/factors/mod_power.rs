/*
  result · b^e ≡ base^exp (mod modulus)
*/

/// Computes (base^exp) mod modulus using fast exponentiation.
/// All parameters are i64 for safety and consistency.
///
/// - `modulus`: the modulo value
/// - `base`: the base number
/// - `exp`: the exponent
pub fn mod_pow(modulus: u64, base: u64, exp: u64) -> u64 {
    assert!(modulus > 0, "modulus cannot be zero"); // avoid division by zero

    if modulus == 1 {
        return 0;
    }

    let mut result: u64 = 1; // keeps track of result
    let mut b = base % modulus; // current power
    let mut e = exp; // exponent

    // process the exponents bit one by one
    while e > 0 {
        if e % 2 == 1 {
            // odd number means bit is 1 -> append to result
            result = result * b % modulus;
        }
        // square b and reduce e to shift right by one bit
        b = (b * b) % modulus;
        e /= 2;
    }

    result
}
