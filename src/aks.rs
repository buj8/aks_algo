use super::math_utils;
use num_bigint::BigUint;

pub fn aks(n: u64) -> bool {
    // Step 1: Check if n is a perfect power
    let n_big = BigUint::from(n);
    if math_utils::check_perfect_power(&n_big).is_some() {
        return false;
    }

    // Step 2: Calculate r.
    // TBI

    // Step 3: For all 1 < a < r, check if (x + a)^n = x^n + a (mod x^r - 1, n)
    // TBI

    // Step 4: If n <= r, output PRIME
    // TBI

    // Step 5: For a = 1 to sqrt(phi(r)) * log(n), check if (x - a)^n != x^n - a (mod x^r - 1, n)
    // TBI

    true
}
