use num::{
    bigint::{BigUint, RandBigInt}, 
    Zero
};
use rand;
use rayon::prelude::{ParallelBridge, ParallelIterator};

lazy_static! {
    static ref PRIMES: Vec<BigUint> = first_primes(2000);
}

pub fn gen_prime(bit_size: u64) -> (usize, BigUint) {
    RandomPrimeCandidate::new(bit_size) 
        .into_iter()
        .enumerate()
        .par_bridge()
        .find_any(|(_i, n)| is_prime(n))
        .expect("Should not be none")
}

struct RandomPrimeCandidate {
    bits: u64,
}

impl RandomPrimeCandidate {
    fn new(bits: u64) -> Self {
        RandomPrimeCandidate { bits }
    }
}

impl Iterator for RandomPrimeCandidate {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        let mut value = rng.gen_biguint(self.bits);

        value.set_bit(0, true);
        value.set_bit(self.bits - 1, true);

        assert_eq!(value.bits(), self.bits);
        Some(value)
    }
}

fn is_prime(n: &BigUint) -> bool {
         
  // Corner cases
  if n <= &1_u8.into(){
      return false;
  }
  if is_first_primable(n) {
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
      if !miller_test(&d, n) {
          return false;
      }
  }
  return true;
}

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

fn first_primes(n: u32) -> Vec<BigUint> {
    fn local_prime(n: u32) -> bool {
        (2..).take_while(|d| (*d) * (*d) <= n).all(|d| n % d != 0)
    }

    (2..n)
        .filter(|d| local_prime(*d))
        .map(|p| BigUint::from(p))
        .collect()
}

fn is_first_primable(candidate: &BigUint) -> bool {
    let zero = &BigUint::zero();
    PRIMES
        .iter()
        .any(|divisor| &(candidate % divisor) == zero)
}

#[cfg(test)]
mod tests {
    use super::*;
  #[test]
  fn test_gen_prime() {
    let (i, prime) = gen_prime(2048);
    println!("Prime found: {:x?} in {} iterations", prime, i);
    // assert!(is_prime(&prime));
  }

  // No real test
  #[test]
  fn test_first_primes() {
    let n = 2000;
    let primes = first_primes(n);
    println!("{:?}", primes);
    // P_n := {p in P | p < n}
    println!("|P_{}| = {}", n, primes.len())
  }
}