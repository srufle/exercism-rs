use num_bigint::BigUint;
use rand::prelude::*;
//  Inspired by https://exercism.org/tracks/rust/exercises/diffie-hellman/solutions/elWanderero
// To deal with "big-primes"
pub fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let base = BigUint::from(base);
    let exponent = BigUint::from(exponent);
    let modulus = BigUint::from(modulus);
    let c = base.modpow(&exponent, &modulus);
    return c.iter_u64_digits().next().unwrap();
}
pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}
