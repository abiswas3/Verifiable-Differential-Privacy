use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
use std::fmt;
// use rand::distributions::{Distribution, Uniform};
use crate::utils::{gen_random, calculate_q, get_generator};

// TODO: Change the primes to BigNum support
pub struct PublicParams {
    pub num_shares: usize,
    pub p: BigNum,
    pub q: BigNum,
    pub g: BigNum,
    pub h: BigNum,
    pub ctx: BigNumContext,    
    pub commitments: Vec<Vec<BigNum>>
}
impl fmt::Display for PublicParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("p", &self.p)
            .field("q", &self.q)
            .field("g", &self.g)
            .field("h", &self.h)
            .finish()
    }
}


impl PublicParams{

    pub fn new(security: i32, num_shares: usize) -> Result<PublicParams, ErrorStack> {
        // create context to manage the bignum
        let mut ctx = BigNumContext::new()?;
   
        // generate prime number with 2*security bits
        let mut p = BigNum::new()?;
        p.generate_prime(2 * security, true, None, None)?;        
        // calculate q from p,such that q | p -1 
        // set q = p - 1/2 and q is guaranteed to be a prime since p is a safe prime
        let q = calculate_q(&p)?;

        // generate random g
        // YOU CANNOT PICK A RANDOM G!!
        // let g = gen_random(&p)?; // That's not how you pick a generator OH FUCK
        // let g = BigNum::from_u32(159)?;        
        let g = get_generator(&p, &q, &mut ctx);
        
        // generate random secret alpha
        let alpha = gen_random(&q)?;
        // calculate h = pow(g, alpha, p)
        let mut h = BigNum::new()?;

        h.mod_exp(&g, &alpha, &p, &mut ctx)?;
        let commitments = Vec::new();

        Ok(Self { num_shares, p, q, g, h, ctx, commitments })

    }
}