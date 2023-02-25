use std::convert::TryFrom;

use num::{bigint::{BigUint, RandBigInt}, BigInt};

fn main() {
  let modulus = BigInt::from(124_u8);
  let mut bigi = BigInt::from(-100_i8);
  let uint = {
    if bigi < BigInt::from(0_i8) {
      bigi = (&bigi % &modulus) + &modulus;
    }
    match BigUint::try_from(bigi) {
        Ok(i) => i,
        Err(i) => panic!("Something happend, that really shouldn't have happened: {:?}", i),
    }
  };
  println!("{:?}", uint)
}