use aks_rs::math_utils::{check_perfect_power, n_order};
use rug::Integer;

#[test]
fn test_check_perfect_power_true_1() {
    // Test 1: 16 is 2^4
    let n = Integer::from(16);
    let result = check_perfect_power(&n);
    assert_eq!(result, Some((Integer::from(2), 4)));
}

#[test]
fn test_check_perfect_power_true_2() {
    // Test 2: 81 is 3^4
    let n = Integer::from(81);
    let result = check_perfect_power(&n);
    assert_eq!(result, Some((Integer::from(3), 4)));
}

#[test]
fn test_check_perfect_power_true_3() {
    // Test 3: 3,486,784,401 is 3^20
    let n = Integer::from(3_486_784_401u64);
    let result = check_perfect_power(&n);
    assert_eq!(result, Some((Integer::from(3), 20)));
}

#[test]
fn test_check_perfect_power_true_4() {
    // Test 4: 1,000,000,000,000 is 10^12
    let n = Integer::from(1_000_000_000_000u64);
    let result = check_perfect_power(&n);
    assert_eq!(result, Some((Integer::from(10), 12)));
}

#[test]
fn test_check_perfect_power_false_1() {
    // Test 1: 15 is not a perfect power
    let n = Integer::from(15);
    let result = check_perfect_power(&n);
    assert_eq!(result, None);
}

#[test]
fn test_check_perfect_power_false_2() {
    // Test 2: 82 is not a perfect power
    let n = Integer::from(82);
    let result = check_perfect_power(&n);
    assert_eq!(result, None);
}

#[test]
fn test_check_perfect_power_false_3() {
    // Test 3: 3,486,784,402 is not a perfect power
    let n = Integer::from(3_486_784_402u64);
    let result = check_perfect_power(&n);
    assert_eq!(result, None);
}

#[test]
fn test_check_perfect_power_false_4() {
    // Test 4: 1,000,000,001 is not a perfect power
    let n = Integer::from(1_000_000_000_001u64);
    let result = check_perfect_power(&n);
    assert_eq!(result, None);
}

#[test]
fn test_n_order_1() {
    // Test 1: r = 2, n = 10
    let r = Integer::from(3);
    let n = Integer::from(10);
    let result = n_order(r, n);
    assert_eq!(result, Ok(Integer::from(4)));
}

#[test]
fn test_n_order_2() {
    // Test 2: r = 3, n = 7
    let r = Integer::from(3);
    let n = Integer::from(7);
    let result = n_order(r, n);
    assert_eq!(result, Ok(Integer::from(6)));
}

#[test]
fn test_n_order_3() {
    // Test 3: r = 5, n = 12
    let r = Integer::from(5);
    let n = Integer::from(12);
    let result = n_order(r, n);
    assert_eq!(result, Ok(Integer::from(2)));
}

#[test]
fn test_n_order_4() {
    // Test 4: r = 7, n = 15
    let r = Integer::from(7);
    let n = Integer::from(15);
    let result = n_order(r, n);
    assert_eq!(result, Ok(Integer::from(4)));
}

#[test]
fn test_n_order_5() {
    // Test 5: r = 2103, n = 3101
    let r = Integer::from(2103);
    let n = Integer::from(3101);
    let result = n_order(r, n);
    assert_eq!(result, Ok(Integer::from(1326)));
}

#[test]
fn test_n_order_6() {
    // Test 6: r = 23409238414123, n = 39318293
    let r = Integer::from(23409238414123u64);
    let n = Integer::from(39318293u64);
    let result = n_order(r, n);
    assert_eq!(result, Ok(Integer::from(88638)));
}

#[test]
fn test_n_order_7() {
    // Test 7: r = 9812393812890371280237891263791238912, n = 123912830127941239312893
    let r = Integer::from_str_radix("9812393812890371280237891263791238912", 10).unwrap();
    let n = Integer::from_str_radix("123912830127941239312893", 10).unwrap();
    let result = n_order(r, n);
    assert_eq!(
        result,
        Ok(Integer::from_str_radix("5633516015958951504", 10).unwrap())
    );
}
