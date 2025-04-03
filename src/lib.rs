use num_bigint::{BigUint, RandBigInt};
use rand;

/// alpha^x mod p
/// output = n^exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint{
    n.modpow(exponent, modulus)
}

/// output = s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    // if *k >= c * x {
    //     return (k - c * x) % q;
    // }
    // return (q + k - (c * x % q)) % q;
    return (k + q - (c * x % q)) % q;
}

/// cond1: r1 = alpha^s * y1^c mod p
/// cond2: r2 = beta^s * y2^c mod p
pub fn verify(y1: &BigUint, y2: &BigUint, r1: &BigUint, r2: &BigUint, alpha: &BigUint, beta: &BigUint, c: &BigUint, s: &BigUint, p: &BigUint) -> bool {
    let cond1 = r1 == &((alpha.modpow(s, p) * y1.modpow(c, p)) % p);
    let cond2 = r2 == &((beta.modpow(s, p) * y2.modpow(c, p)) % p);
    cond1 && cond2
}

pub fn generate_random_less_than(bound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    
    rng.gen_biguint_below(bound)
}

#[cfg(test)]
mod test{
    use num_bigint::BigUint;
    use super::*;

    #[test]
    fn test_toy_example(){
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        
        let x = BigUint::from(6u32);
        let k: BigUint = BigUint::from(7u32);
        let c: BigUint = BigUint::from(4u32);

        let y1 = exponentiate(&alpha, &x, &p);
        let y2 = exponentiate(&beta, &x, &p);

        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = exponentiate(&alpha, &k, &p);
        let r2 = exponentiate(&beta, &k, &p);

        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = solve(&k, &c, &x, &q);
        assert_eq!(s, BigUint::from(5u32));

        let result = verify(&y1, &y2, &r1, &r2, &alpha, &beta, &c, &s, &p);
        assert!(result);
    }

    #[test]
    fn test_toy_example_with_random_numbers(){
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        
        let x = BigUint::from(6u32);
        let k: BigUint = generate_random_less_than(&q);
        let c: BigUint = generate_random_less_than(&q);

        let y1 = exponentiate(&alpha, &x, &p);
        let y2 = exponentiate(&beta, &x, &p);


        let r1 = exponentiate(&alpha, &k, &p);
        let r2 = exponentiate(&beta, &k, &p);


        let s = solve(&k, &c, &x, &q);

        let result = verify(&y1, &y2, &r1, &r2, &alpha, &beta, &c, &s, &p);
        assert!(result);
    }
}
