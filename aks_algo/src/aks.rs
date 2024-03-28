pub fn aks(n : u64) -> bool {
    // Step 1: Check if n is a perfect power
    let n_big = BigUint::from(n);
    if let Some(_) = check_perfect_power(n_big.clone()) {
        return false;
    }

    // Step 2: Calculate r.
    let r = calc_r(n_big.clone());

    true 
}