// use std::ops::Rem;

use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;

pub fn gen_random(limit: &BigNum) -> Result<BigNum, ErrorStack> {
    // generate random bignum between 1, limit-1
    let one = BigNum::from_u32(1)?;
    let mut r = BigNum::new()?;
    let mut tmp1 = BigNum::new()?;


    tmp1.checked_sub(limit, &one)?;
    let mut tmp2 = BigNum::new()?;
    tmp2.checked_add(&r, &one)?;
    tmp1.rand_range(&mut r)?;
    Ok(r)
}

pub fn get_generator(p:& BigNum, q:&BigNum, ctx: &mut BigNumContext)->BigNum{

    let mut a = gen_random(q).unwrap();
    while  mod_exp(&a, q, p, ctx) != BigNum::from_u32(1).unwrap(){
        a = gen_random(q).unwrap();
    }
    return a;
}

pub fn calculate_q(p: &BigNum) -> Result<BigNum, ErrorStack> {
    
    let one = BigNum::from_u32(1)?;
    let two = BigNum::from_u32(2)?;        
    let q = &(p - &one) /&two;    
    Ok(q)

    // let mut q = BigNum::new()?;
    // let one = BigNum::from_u32(1)?;
    // let two = BigNum::from_u32(2)?;
    // let mut tmp = BigNum::new()?;
    // tmp.checked_mul(p, &two, ctx)?;
    // q.checked_add(&tmp, &one)?;
    // Ok(q)
}

pub fn mod_exp(g: &BigNum, x: &BigNum,q: &BigNum, ctx: &mut BigNumContext)->BigNum{
    let mut tmp = BigNum::new().unwrap();
    _ = tmp.mod_exp(g, x, q, ctx).unwrap();
    return tmp;
}
