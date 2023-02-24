use std::convert::TryFrom;

use num::{bigint::{BigUint, RandBigInt}, BigInt};
use rand;
#[derive(Debug)]
struct KeyPair {
    exponent: BigUint,
    modulus: BigUint
}

#[derive(Debug)]
struct RSA {
    pub_key: KeyPair,
    priv_key: KeyPair,
}

impl RSA {

    fn new(key_size: u64) -> RSA {
        let (p, q) = (RSA::gen_biguint_prime(key_size), RSA::gen_biguint_prime(key_size));

        let n = &p * &q;
        let phi_n = (&p - (1 as u8)) * (&q - (1 as u8));

        let e = BigUint::from(65537_u32);
        let d = {
            let mut d0 = RSA::ext_gcd(&e, &phi_n);
            let modulus: BigInt = BigInt::from_biguint(num::bigint::Sign::Plus, phi_n.clone());
            if d0 < BigInt::from(0_i8) {
                d0 = (&d0 % &modulus) + &modulus;
              }
              match BigUint::try_from(d0) {
                  Ok(i) => i,
                  Err(_) => BigUint::from(0_u8),
              }
        };

        RSA {
            pub_key: KeyPair{exponent: e, modulus: n.clone()},
            priv_key: KeyPair{exponent: d, modulus: n.clone()},
        }
    }

    fn crypt(key_pair: &KeyPair, m: BigUint) -> BigUint {
        m.modpow(&key_pair.exponent, &key_pair.modulus)
    }

    // Langsam! TODO(): Miller Rabin Test
    fn is_prime(n: &BigUint) -> bool {
        if n <= &BigUint::from(1_u8) {
            return false;
        }
        let mut p = BigUint::from(2_u8);
        
        while &p <= &n.sqrt() {
            if n % &p == BigUint::from(0_u8) {
                return false; 
            }
            p = &p + 1_u8;
        }
        true
    }

    // fn is_prime(a: &BigUint) -> bool {
    //     true
    // }

    fn ext_gcd(a: &BigUint, b: &BigUint) -> BigInt {
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
    
        if b != &BigUint::from(0_u8) {
            (old_g - old_s * big_a) / big_b
        } 
        else { 
            BigInt::from(0_u8)
        }
    }

    fn gen_biguint_prime(bit_size: u64) -> BigUint {
        let mut rng = rand::thread_rng();

        loop {
            let random: BigUint = rng.gen_biguint(bit_size);
            if RSA::is_prime(&random) {
                return random;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use num::bigint::BigUint;

    use crate::RSA;

    #[test]
    fn test_prime() {
        let prime =  BigUint::from(65537_u32);
        assert!(RSA::is_prime(&prime))
    }

    #[test]
    fn test_random() {
        for _ in 0.. 100 {
            let p = RSA::gen_biguint_prime(16_u64);
            assert!(p % BigUint::from(2_u8) != BigUint::from(0_u8));
        }
    }

    #[test]
    fn test_rsa_keygen() {
        let rsa = RSA::new(16_u64);

        assert!(ggt(rsa.priv_key.exponent, rsa.pub_key.exponent) == BigUint::from(1_u8))
    }

    #[test]
    fn test_rsa_crypt() {
        let rsa = RSA::new(16_u64);
        println!("{:?}", rsa.priv_key);
        println!("{:?}", rsa.pub_key);

        let m = BigUint::from(9238_u16);

        let c = RSA::crypt(&rsa.pub_key, m.clone());

        let m0 = RSA::crypt(&rsa.priv_key, c);
        
        assert_eq!(m, m0);
    }
}
