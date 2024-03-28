use num_bigint::BigUint;
use num_traits::{One, Zero};


// Step 1: Check if n is a perfect power
pub fn check_perfect_power(n: BigUint) -> Option<(BigUint, u32)> {
    let max_base: BigUint = n.nth_root(2);
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
            return Some((base.clone(), exponent));
        }
        base += &one; 
    }
    None
}


// Step 2: Calculate r. r is the smallest integer such that the order of n modulo r is greater than 4 * log(n)^2
pub fn calc_r(n: BigUint) -> u32 {
    let log_n: f64 = (n.bits() as f64).log2();
    let log_n_squared: f64 = log_n.powi(2);
    let mut r: u32 = 1;
    let mut or: BigUint = BigUint::one();
    let four: BigUint = BigUint::from(4u32);

    while or <= (four.clone() * log_n_squared as u32) {
        r += 1;
        or = order_mod_n(r, n.clone());
    }
    r
}

// Auxiliary function to calculate the order of a modulo n
fn order_mod_n(a: u32, n: BigUint) -> BigUint {
    let mut k: u32 = 1;
    let a_big: BigUint = BigUint::from(a);
    let one: BigUint = BigUint::one();
    let zero: BigUint = BigUint::zero();

    while k <= n.bits() as u32 {
        if a_big.clone().pow(k) % n.clone() == one.clone() {
            return BigUint::from(k);
        }
        k += 1;
    }
    zero
}