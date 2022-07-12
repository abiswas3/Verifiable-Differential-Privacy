// use rand;
// use rand::{distributions::Uniform, Rng}; // 0.8.0

use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
use std::fmt;

pub struct PedersenCommitment {
    p: BigNum,
    q: BigNum,
    g: BigNum,
    h: BigNum,
    ctx: BigNumContext,
}

impl fmt::Debug for PedersenCommitment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PedersenCommitment")
            .field("p", &self.p)
            .field("q", &self.q)
            .field("g", &self.g)
            .field("h", &self.h)
            .finish()
    }
}

impl PedersenCommitment {
    pub fn new(security: i32) -> Result<PedersenCommitment, ErrorStack> {
        // create context to manage the bignum
        let mut ctx = BigNumContext::new()?;

        // generate prime number with 2*security bits
        let mut p = BigNum::new()?;
        p.generate_prime(2 * security, false, None, None)?;

        // calculate q from p, where q = 2p + 1
        // this ensures that every element in Z_q is a generator
        let q = calculate_q(&p, &mut ctx)?;

        // generate random g
        let g = gen_random(&p)?;

        // generate random secret alpha
        let alpha = gen_random(&p)?;

        // calculate h = pow(g, alpha, p)
        let mut h = BigNum::new()?;

        h.mod_exp(&g, &alpha, &p, &mut ctx)?;

        Ok(Self { p, q, g, h, ctx })
    }

    pub fn open(&mut self, c: &BigNum, x: u32, args: &[&BigNum]) -> Result<bool, ErrorStack> {
        // c: commitment
        // x: the secret
        // args: array of randomness
        let total = args.iter().fold(BigNum::new()?, |acc, x| &acc + *x);
        let res = self.helper(x, &total)?;
        Ok(&res == c)
    }

    pub fn add(&mut self, cm: &[&BigNum]) -> Result<BigNum, ErrorStack> {
        // Multiply arry of commitments cm
        let res = cm.iter().fold(BigNum::from_u32(1)?, |acc, x| &acc * *x);
        let mut tmp = BigNum::new()?;
        tmp.nnmod(&res, &self.q, &mut self.ctx)?;
        Ok(tmp)
    }

    pub fn commit(&mut self, x: u32) -> Result<(BigNum, BigNum), ErrorStack> {
        let r = gen_random(&self.q)?;
        let c = self.helper(x, &r)?;
        Ok((c, r))
    }

    fn helper(&mut self, x: u32, r: &BigNum) -> Result<BigNum, ErrorStack> {
        // What does this function do ?
        // tmp3 = g^x1
        // tmp4 = h^r
        // c = tmp3*tmp4;
        // returns g^x1h^r
        let x1 = BigNum::from_u32(x)?;
        let mut c = BigNum::new()?;
        let mut tmp3 = BigNum::new()?;
        let mut tmp4 = BigNum::new()?;
        tmp3.mod_exp(&self.g, &x1, &self.q, &mut self.ctx)?;
        tmp4.mod_exp(&self.h, r, &self.q, &mut self.ctx)?;
        c.mod_mul(&tmp3, &tmp4, &self.q, &mut self.ctx)?;
        Ok(c)
    }
}

fn gen_random(limit: &BigNum) -> Result<BigNum, ErrorStack> {
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

fn calculate_q(p: &BigNum, ctx: &mut BigNumContext) -> Result<BigNum, ErrorStack> {
    // generate q = 2p + 1
    let mut q = BigNum::new()?;
    let one = BigNum::from_u32(1)?;
    let two = BigNum::from_u32(2)?;
    let mut tmp = BigNum::new()?;
    tmp.checked_mul(p, &two, ctx)?;
    q.checked_add(&tmp, &one)?;
    Ok(q)
}
