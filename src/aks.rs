use super::math_utils;
use rug::Integer;

pub fn aks(n: &Integer) -> bool {
    // Step 1: Check if n is a perfect power
    if math_utils::check_perfect_power(n).is_some() {
        return false;
    }

    // Step 2: Calculate r.
    let r = math_utils::get_r(n);
    // debug output
    println!("r: {:?}", r);

    // Step 3: For all 1 < a < r, check if (x + a)^n = x^n + a (mod x^r - 1, n)
    //
    // Step 4: If n <= r, output PRIME
    if n <= &r.unwrap() {
        return true;
    }

    // Step 5: For a = 1 to sqrt(phi(r)) * log(n), check if (x - a)^n != x^n - a (mod x^r - 1, n)
    // TBI

    // Step 6: Output PRIME
    true
}
