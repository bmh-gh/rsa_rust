use std::convert::TryFrom;

use num::{bigint::{BigUint, RandBigInt}, BigInt};
use rand;


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
        let (p, q) = (Self::gen_biguint_prime(key_size), Self::gen_biguint_prime(key_size));

        let n = &p * &q;
        let phi_n = (&p - (1_u8)) * (&q - (1_u8));

        let e = BigUint::from(65537_u32);
        let d = Self::modular_inverse(&e, &phi_n);

        Self {
            pub_key: Key{exponent: e, modulus: n.clone()},
            priv_key: Key{exponent: d, modulus: n.clone()}
        }
    }

    // Langsam! TODO(): Miller Rabin Test
    // fn is_prime(n: &BigUint) -> bool {
    //     if n <= &BigUint::from(1_u8) {
    //         return false;
    //     }
    //     let mut p = BigUint::from(2_u8);
    //     let limit = &n.sqrt();
    //     while &p <= limit {
    //         if n % &p == BigUint::from(0_u8) {
    //             return false; 
    //         }
    //         p = &p + 1_u8;
    //     }
    //     true
    // }
    
    fn miller_test(d: &BigUint, n: &BigUint) -> bool {
        let mut d = d.clone();
        // Pick a random number in [2..n-2]
        // Corner cases make sure that n > 4
        let mut rng = rand::thread_rng();
        let a: BigUint = rng.gen_biguint_range(&3_u8.into(), &(BigUint::from(2_u8).pow(n.bits() as u32)));

        // Compute a^d % n
        let mut x = a.modpow(&d, &n);
     
        if x == 1_u8.into() || x == n - 1_u8 {
            return true
        }
     
        // Keep squaring x while one of the
        // following doesn't happen
        // (i) d does not reach n-1
        // (ii) (x^2) % n is not 1
        // (iii) (x^2) % n is not n-1
        while d != (n - BigUint::from(1_u8)) {
            x = (&x * &x) % n;
            d = &d * &BigUint::from(2_u8);
         
            if x == 1_u8.into() {
                return false;
            }
            if x == n - 1_u8 {
                return true;
            }
        }
     
        // Return composite
        return false;
    }

    fn is_prime(n: &BigUint) -> bool {
         
        // Corner cases
        if n <= &1_u8.into() || n == &4_u8.into() {
            return false;
        }
        if n <= &3_u8.into() {
            return true;
        }
     
        // Find r such that n = 2^d * r + 1
        // for some r >= 1
        let mut d = n - BigUint::from(1_u8);
         
        while &d % BigUint::from(2_u8) == BigUint::from(0_u8) {
            d = d / BigUint::from(2_u8);
        }
     
        // Iterate given number of 'k' times
        for _ in 0..3 {
            if !Self::miller_test(&d, n) {
                return false;
            }
        }
        return true;
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
        match BigUint::try_from(old_s) {
            Ok(i) => i,
            Err(_) => panic!("An internal error has ocurred. Please start the program anew :)"),
        }  
    }

    fn gen_biguint_prime(bit_size: u64) -> BigUint {
        let mut rng = rand::thread_rng();

        loop {
            let random: BigUint = dbg!(rng.gen_biguint(bit_size - 1) * BigUint::from(2_u8) + BigUint::from(1_u8));
            if dbg!(Self::is_prime(&random)) {
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
        let prime =  RSA::gen_biguint_prime(2000);
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
    fn test_time() {
        let _rsa = RSA::new(1024);
    }

    #[test]
    fn test_rsa_crypt() {
        let rsa = RSA::new(1024);

        let m = BigUint::from(9238_u16);

        let c = rsa.pub_key.crypt(&m);

        let m0 = rsa.priv_key.crypt(&c);
        
        assert_eq!(m, m0);
    }
}
