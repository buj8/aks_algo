use ecm;
use rug::{ops::Pow, Float, Integer};
use std::collections::HashMap;

// Step 1: Check if n is a perfect power
pub fn check_perfect_power(n: &Integer) -> Option<(Integer, u32)> {
    let two = Integer::from(2);

    if *n <= 1 {
        return None;
    }

    let max_base = n.clone().sqrt();

    let mut base = two;

    while base <= max_base {
        let mut temp = n.clone();
        let mut exponent: u32 = 0;

        while temp.is_divisible(&base) {
            temp /= &base;
            exponent += 1;
        }

        if temp == 1 {
            return Some((base, exponent));
        }

        base += 1;
    }

    None
}

pub fn get_r(n: &Integer) -> Result<Integer, &'static str> {
    let log_n = Float::with_val(53, n).ln();
    let log_squared = log_n.square();

    let mut r = Integer::from(1);

    loop {
        r += 1;

        match n_order(r.clone(), n.clone()) {
            Ok(k) => {
                let k_float = Float::with_val(53, &k);
                if k_float > log_squared {
                    print!("{} = 1 mod {} with order {}\n", n, r, k);
                    break;
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
    Ok(r)
}

/*
This is a port from the n_order method from the sympy library
(https://docs.sympy.org/latest/modules/ntheory.html#sympy.ntheory.residue_ntheory.n_order)

This calculates the multiplicative order of an integer r modulo n.
*/
pub fn n_order(r: Integer, n: Integer) -> Result<Integer, &'static str> {
    if n <= 1 {
        return Err("n should be an integer greater than 1");
    }
    let r = r % &n;
    if r == 1 {
        return Ok(Integer::from(1));
    }
    if r.clone().gcd(&n) != 1 {
        return Err("The two numbers should be relatively prime");
    }

    let factors = factorint(&n);
    let mut order_factors = HashMap::new();

    for (px, kx) in factors {
        if kx > 1 {
            *order_factors.entry(px.clone()).or_default() += kx - 1;
        }
        let sub_factors = factorint(&(px.clone() - 1));
        for (py, ky) in sub_factors {
            *order_factors.entry(py).or_default() += ky;
        }
    }

    let mut order = Integer::from(1);
    for (px, kx) in order_factors.iter() {
        order *= px.clone().pow(*kx as u32);
    }

    for (p, e) in order_factors {
        for _ in 0..e {
            if r.clone().pow_mod(&(order.clone() / &p), &n) == Ok(Integer::from(1)) {
                order /= &p;
            } else {
                break;
            }
        }
    }
    Ok(order)
}

/*
Instead of using the sympy implementation of factorint which uses different algorithms
to factorize the number we will just use the Lenstra elliptic curve factorization
method to factorize the number.
*/
fn factorint(n: &Integer) -> HashMap<Integer, usize> {
    ecm::ecm(n).unwrap()
}
