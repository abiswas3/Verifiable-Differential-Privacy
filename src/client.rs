use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
// use rand::random;
use std::ops::Rem;
use std::fmt;
use crate::utils::{gen_random, mod_exp};

pub struct Client{
    num_servers: usize,
    num_candidates: u32,
    p: BigNum,
    q: BigNum,
    g: BigNum,
    h: BigNum,    
}
pub struct Share{
    pub commitments: Vec<BigNum>,
    pub randomness: Vec<BigNum>,
    pub shares: Vec<BigNum>,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("p", &self.p)
            .field("q", &self.q)
            .field("g", &self.g)
            .field("h", &self.h)
            .finish()
    }
}

impl Client{

    pub fn new(num_servers: usize, num_candidates: u32, _p: &BigNum, _q: &BigNum, _g: &BigNum, _h: &BigNum) -> Client {
       
        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
        
        Self{num_servers, num_candidates, p, q, g, h}
    }

    pub fn vote(&self, vote: u32, ctx: &mut BigNumContext)->Vec::<Share>{

        if vote > self.num_candidates -1 {
            panic!("crash and burn");
        }

        let mut encoded_vote = Vec::<Share>::with_capacity(self.num_candidates as usize);        
        for i in 0..self.num_candidates as usize{
            if i as u32 == vote{
                encoded_vote.push(self.share(1, ctx));
            }
            else{
                encoded_vote.push(self.share(0, ctx));
            }            
        }
        return encoded_vote;
    }

    pub fn share(&self, _secret: u32, ctx: &mut BigNumContext)->Share{

        let mut shares = Vec::new();
        let mut commitments = Vec::new();
        let mut randomness = Vec::new();

        for _ in 1..(self.num_servers){            
            let tmp = gen_random(&self.q).unwrap(); 
            let (com, r) = self.commit(&tmp, ctx).unwrap();

            shares.push(tmp);
            commitments.push(com);
            randomness.push(r); 
        }
        
        let secret = BigNum::from_u32(_secret).unwrap();
        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut last_share = BigNum::new().unwrap();
        _ = last_share.mod_sub(&secret, &total, &self.q, ctx);
        
        let (com, r) = self.commit(&last_share, ctx).unwrap(); // FIX THIS
        shares.push(last_share);
        commitments.push(com);
        randomness.push(r); 

        return Share{commitments: commitments, 
            randomness: randomness,
            shares: shares
        };
    }

    pub fn commit(&self, x: &BigNum,  ctx: &mut BigNumContext) -> Result<(BigNum, BigNum), ErrorStack> {
        let r = gen_random(&self.q).unwrap();
        let c = self.helper(&x, &r, ctx)?;
        Ok((c, r))
    }

    pub fn helper(& self, x1: &BigNum, r: &BigNum, ctx: &mut BigNumContext) -> Result<BigNum, ErrorStack> {
        // returns g^x1h^r        
        let tmp3 = mod_exp(&self.g, x1, &self.p, ctx);
        let tmp4 = mod_exp(&self.h, r, &self.p, ctx);                
        return Ok((&(tmp3) * &(tmp4)).rem(&self.p));        
    }  
    pub fn mult_commitments(&mut self, cm: &[&BigNum]) -> Result<BigNum, ErrorStack> {
        // Multiply arry of commitments cm
        let res = (cm.iter().fold(BigNum::from_u32(1)?, |acc, x| &acc * *x)).rem(&self.p);
        Ok(res)
    }    

}