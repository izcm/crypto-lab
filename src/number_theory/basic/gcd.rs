/*
 Invariant: r_k = x_k * a + y_k * b
 Where k is the current iteration and a, b are fixed
*/

/// Computes the extended GCD of two integers `a` and `b`.`
///
/// - `a` is the first input integer
/// - `b` is the second input integer
///
/// Special cases:
/// - If `b == 0`, returns `a.abs()`
pub fn gcd(a: i64, b: i64) -> i64 {
    // avoids dividing by 0
    if b == 0 {
        return a.abs(); // convention calls for positive GCD
    }

    let mut x = a;
    let mut y = b;

    // equal numbers => GCD = themselves
    if x == y {
        return x;
    }

    if y > x {
        std::mem::swap(&mut x, &mut y);
    }

    while y > 0 {
        let new = x;
        x = y;
        y = new % y;
    }

    return x;
}

/// Computes the extended GCD of two integers `a` and `b`.
///
/// Returns a triple `(g, u, v)` such that:
///
/// ```text
/// g = gcd(a, b)
/// g = a*u + b*v
/// ```
///
/// - `a` is the first input integer
/// - `b` is the second input integer
/// - `g` is always nonnegative
/// - `u` and `v` are integers that satisfy Bézout's identity
///
/// Special cases:
/// - If `b == 0`, returns `(a.abs(), a.signum(), 0)`
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    // avoids dividing by 0
    // returns `v` set to 0, absolute value of `a` as GCD, `u` is set to -+1 depending on the sign of `a`
    if b == 0 {
        return (a.abs(), a.signum(), 0);
    }

    let mut r0 = a;
    let mut r1 = b;

    let mut x0 = 1; // coeff for a
    let mut x1 = 0; // forward coeff for a

    while r1 != 0 {
        let q = r0 / r1;

        let new_r = r0 - q * r1;
        r0 = r1;
        r1 = new_r;

        let new_x = x0 - q * x1;
        x0 = x1;
        x1 = new_x;
    }

    let g = r0;
    let x = x0;
    let y = (g - a * x) / b;

    // g is the GCD, x = u and y = v
    (g, x, y)
}
