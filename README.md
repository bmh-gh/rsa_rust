# RSA Cryptography Project Overview

## Caution: The project is not suitable for production!!! 

This project implements the RSA public-key cryptographic algorithm in Rust.  The code provides functions to generate RSA key pairs and to encrypt and decrypt messages using those keys.

## RSA Algorithm

RSA is a public-key cryptographic algorithm that uses a pair of keys, a public key and a private key, to encrypt and decrypt messages. The public key can be shared with anyone, while the private key is kept secret. Messages are encrypted using the public key and can only be decrypted using the corresponding private key.

The basic steps for RSA are:

  1. Choose two large prime numbers, $p$ and $q$\
  To code this we basically use a CPRNG (Cryptographic Secure Number Generator) and generate these two prime numbers. Given a bit size k for the key, the primes only have to be half as big as the k ($2^k = 2^{\frac{k}{2}} \cdot 2^{\frac{k}{2}}$)
  
    // using the 'prime' module
    (p, q) = (prime::gen_prime(key_size / 2), prime::gen_prime(key_size / 2));

  2. Calculate the modulus $n = p \cdot q$

  3. Calculate Euler's totient function\
   $\varphi(n) = \prod_{i = 1}^n (p^{k_i} \cdot p^{k_{i-1}}) = (p - 1) \cdot (q - 1)$

  4. Choose an integer in range from 1..$\varphi(n)$ that is relatively prime to $\varphi(n)$:\
  $e \in \Z _ {\varphi(n)} \backslash \{ 0 \}: gcd(e, \varphi(n)) = 1$.\
  \
  Because $e$ is part of the public key it is not necessary to keep it secret. Therefore you can safe computing power if you just set it to a fixed value. Commonly used is $e = 65537 = 2^{16} + 1$. So it is necessary that the given bitsize for the key is at least $k$ ($|p| = \frac{k}{2}$ $\land$ $|q| = \frac{k}{2}$ $\implies$ $|(p - 1) \cdot (q - 1)| = k$).

  5. Calculate the modular inverse $d$ of $e$ modulo $\varphi(n)$ such that $d \cdot e ≡ 1$ $mod$ $\varphi(n) \iff e \equiv d^{-1}$ $mod$ $\varphi(n)$\
  To compute the modular inverse I use the extended eucledean algortihm. The output of the eucledean algorithm is a tuple of 4 values (r, q, s, t) where t is the modular inverse. There is a possibility that t is negative. If so you need to use the laws of modular arithmetic and make it positive.

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

  6. The public key is the pair $k_{pub} := (e, n)$ and the private key is the pair $k_{priv} := (d, n)$

To encrypt a message $m$ using $k_{pub}$, the ciphertext $c$ is calculated as $c \equiv m^{k_{pub}}$ $mod$ $n$. To decrypt the ciphertext $c$ using $k_{priv}$, the original message $m$ is recovered as $m \equiv c^{k_{priv}}$ $mod$ $n$. \
To generate a new key pair you just call the function `new(bitsize)` which takes the bitsize of the key as an argument. It returns a struct which contains the keypair with the functionality to encrypt and decrypt input, depending on which key pair you use.\
Normally the exponentiation in the equation is a really expensive operation which needs n steps to finish (where n is the number of the exponent). The library `BigInt` in Rust provides a functionality to use the square-and-multiply algorithm which only needs log(n) steps to finish the modular exponentiation.

## Prime number generation

The prime number generation is soon to be shifted to another repository because it is needed in many more projects 

## Weaknesses

This still to be done!

## Trivia

  - The RSA algorithm was invented by Ron Rivest, Adi Shamir, and Leonard Adleman in 1977.
  - RSA is widely used for secure data transmission, including in HTTPS, SSL/TLS, and SSH protocols.
  - The largest RSA key size currently in use is 4096 bits.