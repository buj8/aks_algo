use num_bigint::BigUint;
use num_traits::{One, Zero};

// Step 1: Check if n is a perfect power
pub fn check_perfect_power(n: &BigUint) -> Option<(BigUint, u32)> {
    let max_base: BigUint = n.nth_root(2); // This could be optimized for large n
    let one: BigUint = BigUint::one();
    let two: BigUint = &one + &one;
    let mut base: BigUint = two.clone();

    while base <= max_base {
        let mut temp: BigUint = n.clone();
        let mut exponent: u32 = 0;
        while &temp % &base == BigUint::zero() {
            temp /= &base;
            exponent += 1;
        }
        if temp == BigUint::one() {
            return Some((base, exponent));
        }
        base += &one;
    }
    None
}

// Step 2: Calculate r. r is the smallest integer such that the order of n modulo r is greater than 4 * log(n)^2
