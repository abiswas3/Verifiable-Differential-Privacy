use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
use crate::utils::{gen_random, mod_exp};
use std::ops::Rem;

pub struct Commitment{
    p: BigNum,
    q: BigNum,
    g: BigNum,
    h: BigNum,
}

impl Commitment {

    pub fn new(_p: &BigNum, _q: &BigNum, _g: &BigNum, _h: &BigNum)->Commitment{

        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
        Self{p, q, g, h}

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
    pub fn open(&self, c: &BigNum, x: &BigNum, r: &BigNum, ctx: &mut BigNumContext) -> Result<bool, ErrorStack> {
        // c: commitment
        // x: the secret
        // r: array of randomness
        
        let res = self.helper(&x, &r, ctx)?;    
        Ok(&res == c)
    } 

    pub fn commit(&self, x: &BigNum,  ctx: &mut BigNumContext) -> Result<(BigNum, BigNum), ErrorStack> {

        let r = gen_random(&self.q).unwrap();
        let c = self.helper(&x, &r, ctx)?;
        Ok((c, r))
    }    
}