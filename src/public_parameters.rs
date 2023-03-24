use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
use std::fmt;
// use rand::distributions::{Distribution, Uniform};

pub struct PublicParams {
    pub num_shares: usize,    
    pub q: BigNum,
    pub ctx: BigNumContext,    
}
impl fmt::Display for PublicParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("q", &self.q)
            .finish()
    }
}


impl PublicParams{

    pub fn new(security: i32, num_shares: usize) -> Result<PublicParams, ErrorStack> {
        
        println!("Generating Public Parameters");
        
        // create context to manage the bignum
        let ctx = BigNumContext::new()?;
   
        // generate prime number with 2*security bits
        let mut q = BigNum::new()?;        
        q.generate_prime(security, true, None, None)?;        
        println!("Done\n");
        Ok(Self { num_shares, q, ctx })
        
    }
}