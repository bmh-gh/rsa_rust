use std::convert::TryFrom;
use num::{
    bigint::BigUint, 
    BigInt
};

#[macro_use]
extern crate lazy_static;

mod prime;

#[derive(Debug)]
struct Key {
    exponent: BigUint,
    modulus: BigUint
}

impl Key {
    fn crypt(&self, m: &BigUint) -> BigUint {
        m.modpow(&self.exponent, &self.modulus)
    }
}

#[derive(Debug)]
struct RSA {
    pub_key: Key,
    priv_key: Key,
}

impl RSA {

    fn new(key_size: u64) -> Self {
        let ((_, p), (_, q)) = (prime::gen_prime(key_size / 2), prime::gen_prime(key_size / 2));

        let n = &p * &q;
        let phi_n = (&p - (1_u8)) * (&q - (1_u8));

        let e = BigUint::from(65537_u32);
        let d = Self::modular_inverse(&e, &phi_n);

        Self {
            pub_key: Key{exponent: e, modulus: n.clone()},
            priv_key: Key{exponent: d, modulus: n.clone()}
        }
    }

    fn modular_inverse(a: &BigUint, b: &BigUint) -> BigUint {
        // Based on the extended eucledean algorithm
        let big_a = BigInt::from_biguint(num::bigint::Sign::Plus, a.clone());
        let big_b = BigInt::from_biguint(num::bigint::Sign::Plus, b.clone());
        let (mut s, mut old_s) = (BigInt::from(0_i8), BigInt::from(1_i8));
        let (mut g, mut old_g) = (big_b.clone(), big_a.clone());

        while g != BigInt::from(0_u8) {
            let q = &old_g / &g;
            let (new_r, new_s) = (&old_g - &q * &g, old_s - &q * &s);
            old_g = g;
            g = new_r;
            old_s = s;
            s = new_s;
        }
        // Normalizing the moudular inverse of the input
        if old_s < BigInt::from(0_i8) {
            old_s = (&old_s % &big_b) + &big_b;
        }
        BigUint::try_from(old_s).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use num::BigUint;

    use crate::RSA;

    #[test]
    fn test_rsa() {
        // Works throgh following proof:
        let rsa = RSA::new(4096);
        let m = BigUint::from(234_u8);
        println!("pub := {:x?}", rsa.pub_key);
        println!("priv := {:x?}", rsa.priv_key);
        assert_eq!(m, rsa.priv_key.crypt(&rsa.pub_key.crypt(&m)))
    }
}
