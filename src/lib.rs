use rayon::prelude::*;
use std::convert::TryFrom;

use num::{
    bigint::{BigUint, RandBigInt},
    BigInt, Zero,
};
use rand::{self};

#[derive(Debug)]
pub struct Key {
    exponent: BigUint,
    modulus: BigUint,
}

impl Key {
    fn crypt(&self, m: &BigUint) -> BigUint {
        m.modpow(&self.exponent, &self.modulus)
    }
}

fn first_primes(n: u32) -> Vec<BigUint> {
    fn local_prime(n: u32) -> bool {
        (2..).take_while(|d| (*d) * (*d) <= n).all(|d| n % d != 0)
    }

    (2..n)
        .filter(|d| local_prime(*d))
        .map(|p| BigUint::from(p))
        .collect()
}

pub fn is_first_primable(candidate: &BigUint) -> bool {
    let null = &BigUint::zero();
    // Kann sein, dass das hier nicht ganz richtig ist.
    PRIMES
        .iter()
        // .any(|divisor| &(candidate % divisor) == null && &(divisor * divisor) <= candidate)
        .any(|divisor| &(candidate % divisor) == null)
}

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref PRIMES: Vec<BigUint> = first_primes(4000);
}

#[derive(Debug)]
pub struct RSA {
    pub_key: Key,
    priv_key: Key,
}

impl RSA {
    pub fn new(key_size: u64) -> Self {
        let ((_, p), (_, q)) = (
            Self::generate_prime(key_size),
            Self::generate_prime(key_size),
        );

        let n = &p * &q;
        let phi_n = (&p - (1_u8)) * (&q - (1_u8));

        let e = BigUint::from(65537_u32);
        let d = Self::modular_inverse(&e, &phi_n);

        Self {
            pub_key: Key {
                exponent: e,
                modulus: n.clone(),
            },
            priv_key: Key {
                exponent: d,
                modulus: n.clone(),
            },
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
        let a: BigUint =
            rng.gen_biguint_range(&3_u8.into(), &(BigUint::from(2_u8).pow(n.bits() as u32)));

        // Compute a^d % n
        let mut x = a.modpow(&d, &n);

        if x == 1_u8.into() || x == n - 1_u8 {
            return true;
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

        if is_first_primable(n) {
            return false;
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

    /// Generates a prime number
    ///
    /// -mh- lösch das oder übersetze.
    /// Ich habe das mal etwas refaktorisiert: Es gibt ein
    /// [RandomBigUintOdds] Objekt, das einen Iterator
    /// erzeugen kann, der unendlich viele ungrade Zufallszahlen
    /// erzeugt. Die Ausgabe wird enumerisiert, d.h. es wird einfach
    /// ein fortlaufender Index für jede Zufallszahl geschaffen, um
    /// später herauszufinden, wieviel Versuche benötigt werden, bis
    /// eine Primzahl gefunden wurde. Nur zu statistischen Zwecken.
    /// Anschließend wird die Primzahl-Prüfung parallelisiert.
    /// Der Iterator wird in einen ParallelIterator mit Hilfe von
    /// [IterBridge] umgewandelt. [find_first] bricht die Iteration
    /// ab, sobald eine Primzahl gefunden wurde. Mit [expect] wird
    /// dann aus dem Option die Primzahl (und der Iterationswert)
    /// herausgenommen. Es kann (darf) nicht passieren, dass keine
    /// Primzahl gefunden wurde, weil der ursprüngliche Iterator
    /// unendlich viele Zahlen generiert und eine davon muss halt
    /// eine Primzahl sein.
    ///
    /// BTW: Ich habe den ursprünglichen Namen geändert. Wenn es
    /// keinen Grund gibt, sollten Implementierungsdetails kein
    /// Bestandteil eines Funktionsnamens sein. Der Typ im
    /// Funktionsnamen ist ein Implementierungsdetail. Außerdem
    /// erschließt sich dieser aus der Signatur, weil die Funktion
    /// ein BigUint zurück gibt.
    fn generate_prime(bit_size: u64) -> (usize, BigUint) {
        RandomPrimeCandidates::new(bit_size)
            .into_iter()
            .enumerate()
            .par_bridge()
            // .find_first(|(_iteration, n)| Self::is_prime(n))
            .find_any(|(_iteration, n)| Self::is_prime(n))
            .expect("iterator should not return None")
    }
}

struct RandomPrimeCandidates {
    bits: u64,
    // rng: ThreadRng,
}

impl RandomPrimeCandidates {
    fn new(bits: u64) -> Self {
        RandomPrimeCandidates { bits }
    }
}

impl Iterator for RandomPrimeCandidates {
    type Item = BigUint;

    /// Erzeugt unendliche viele ungerade Zufallszahlen zwischen
    /// 2^(self.bits - 1) + 1 und (2^self.bits)-1
    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        // let value = rng.gen_biguint(self.bitsize - 1) * BigUint::from(2_u8) + BigUint::from(1_u8);
        let mut value = rng.gen_biguint(self.bits);
        let before = value.bits();
        // value > 2^(bitsize / 2)
        value.set_bit(0, true);
        value.set_bit(self.bits - 1, true);
        let after = value.bits();
        assert!(before <= after);
        assert_eq!(value.bits(), self.bits);
        // Setze das niedrigste Bit => value % 2 == 1 (ungerade)
        Some(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::RSA;
    use num::bigint::BigUint;

    #[test]
    fn test_prime() {
        let (iterations, prime) = RSA::generate_prime(2000);
        println!("Iterationen {}", iterations);
        assert!(RSA::is_prime(&prime))
    }

    #[test]
    fn test_random() {
        for _ in 0..100 {
            let (_, p) = RSA::generate_prime(100_u64);
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

    #[test]
    fn test_keylen() {
        // Für einen 4096-Bit-RSA-Schlüssel werden zwei Primzahlen
        // benötigt, die jeweils etwas 2048 Bits lang sind. Das
        // liegt daran, dass die Länge des RSA-Schlüssels die Länge
        // des Produkts der beiden Primzahlen ist. In der Regel
        // sollten die beiden Primzahlen ähnlich groß sein, um zu
        // verhindern, dass der Schlüssel durch Faktorisierung
        // gebrochen wird. -chatgpt-
        for bits in vec![
            1024, 2048, 3072,
            // 4096,
        ] {
            let rsa = RSA::new(bits);
            println!("RSA mit {} bit Primzahlen", bits);
            println!(
                "Öffentlicher Schlüssel: (e {} bits, N {} bits)",
                rsa.pub_key.exponent.bits(),
                rsa.pub_key.modulus.bits()
            );

            println!(
                "Privater Schlüssel: (d {} bits, N {} bits",
                rsa.priv_key.exponent.bits(),
                rsa.priv_key.modulus.bits(),
            );
        }
    }

    #[test]
    fn test_first_primes() {
        let subject = first_primes(4000);
        println!("{:?}", subject);
        println!(
            "Anzahl der Primzahlen im Interval ({}..{}] ist {}",
            0,
            1000,
            subject.len()
        );
    }

    #[test]
    fn test_is_first_primable() {
        assert!(is_first_primable(&(3347u16 * 4).into()))
    }

    #[test]
    fn test_random_numbers_are_odd() {
        let zwei = &BigUint::from(2u8);
        let eins = &BigUint::from(1u8);
        let result = RandomPrimeCandidates::new(1024)
            .into_iter()
            .take(10)
            .all(|r| &(r % zwei) == eins);

        assert!(result)
    }

    #[test]
    fn test_random_numbers_are_big() {
        let zwei = &BigUint::from(2u8);
        for bits in vec![1024_u32, 2048, 3072, 4096] {
            let result = RandomPrimeCandidates::new(bits as u64)
                .into_iter()
                .take(10)
                .all(|r| r > zwei.pow(bits / 2));

            assert!(result)
        }
    }
}
