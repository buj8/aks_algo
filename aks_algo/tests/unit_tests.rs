use aks_algo::math_utils::check_perfect_power;
use num_bigint::BigUint;


#[test]
fn test_check_perfect_power_true_1() {
    // Test 1: 16 is 2^4
    let n = BigUint::from(16u32);
    let result = check_perfect_power(n.clone());
    assert_eq!(result, Some((BigUint::from(2u32), 4)));
}

#[test]
fn test_check_perfect_power_true_2() {
    // Test 2: 81 is 3^4
    let n = BigUint::from(81u32);
    let result = check_perfect_power(n.clone());
    assert_eq!(result, Some((BigUint::from(3u32), 4)));
}

#[test]
fn test_check_perfect_power_true_3() {
    // Test 3: 3,486,784,401 is 3^20
    let n = BigUint::from(3_486_784_401u32);
    let result = check_perfect_power(n.clone());
    assert_eq!(result, Some((BigUint::from(3u32), 20)));
}

#[test]
fn test_check_perfect_power_true_4() {
    // Test 4: 1,000,000,000,000 is 10^12
    let n = BigUint::from(1_000_000_000_000u64);
    let result = check_perfect_power(n.clone());
    assert_eq!(result, Some((BigUint::from(10u32), 12)));
}

#[test]
fn test_check_perfect_power_false_1() {
    // Test 1: 15 is not a perfect power
    let n = BigUint::from(15u32);
    let result = check_perfect_power(n.clone());
    assert_eq!(result, None);
}

#[test]
fn test_check_perfect_power_false_2() {
    // Test 2: 82 is not a perfect power
    let n = BigUint::from(82u32);
    let result = check_perfect_power(n.clone());
    assert_eq!(result, None);
}


#[test]
fn test_check_perfect_power_false_3() {
    // Test 3: 3,486,784,402 is not a perfect power
    let n = BigUint::from(3_486_784_402u32);
    let result = check_perfect_power(n.clone());
    assert_eq!(result, None);
}

#[test]
fn test_check_perfect_power_false_4() {
    // Test 4: 1,000,000,001 is not a perfect power
    let n = BigUint::from(1_000_000_000_001u64);
    let result = check_perfect_power(n.clone());
    assert_eq!(result, None);
}