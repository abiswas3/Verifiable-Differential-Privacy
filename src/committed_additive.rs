use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
// use std::fmt;
// use rand::distributions::{Distribution, Uniform};

// TODO: Change the primes to BigNum support
pub struct CommitedAdditiveSecretSharing {
    pub num_shares: usize,
    pub p: BigNum,
    pub q: BigNum,
    pub g: BigNum,
    pub h: BigNum,
    ctx: BigNumContext,    
}

pub struct Share{
    pub commitments: Vec<BigNum>,
    pub randomness: Vec<BigNum>,
    pub shares: Vec<BigNum>,
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


impl CommitedAdditiveSecretSharing{

    pub fn new(security: i32, num_shares: usize) -> Result<CommitedAdditiveSecretSharing, ErrorStack> {
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

        Ok(Self { num_shares, p, q, g, h, ctx })
    }


    pub fn share(&mut self, _secret: u32)->Share{

        let mut shares = Vec::new();
        let mut commitments = Vec::new();
        let mut randomness = Vec::new();

        for _ in 1..(self.num_shares){            
            let tmp =gen_random(&self.q).unwrap(); 
            let (com, r) = self.commit(&tmp).unwrap();

            shares.push(tmp);
            commitments.push(com);
            randomness.push(r); 
        }
        
        let secret = BigNum::from_u32(_secret).unwrap();
        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut last_share = BigNum::new().unwrap();
        _ = last_share.mod_sub(&secret, &total, &self.q, &mut self.ctx);
        
        let (com, r) = self.commit(&last_share).unwrap(); // FIX THIS
        shares.push(last_share);
        commitments.push(com);
        randomness.push(r); 

        return Share{commitments: commitments, 
            randomness: randomness,
            shares: shares
        };
    }

    pub fn reconstruct(&mut self, shares: Vec<BigNum>)->BigNum{

        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut reconstructed_value = BigNum::new().unwrap();
        _ = reconstructed_value.nnmod(&total, &self.q, &mut self.ctx);
        return reconstructed_value;
    }


    fn helper(&mut self, x1: &BigNum, r: &BigNum) -> Result<BigNum, ErrorStack> {
        // returns g^x1h^r
        
        let mut c = BigNum::new()?;
        let mut tmp3 = BigNum::new()?;
        let mut tmp4 = BigNum::new()?;
        tmp3.mod_exp(&self.g, &x1, &self.q, &mut self.ctx)?;
        tmp4.mod_exp(&self.h, r, &self.q, &mut self.ctx)?;
        c.mod_mul(&tmp3, &tmp4, &self.q, &mut self.ctx)?;
        return Ok(c);
    }  
    
    pub fn commit(&mut self, x: &BigNum) -> Result<(BigNum, BigNum), ErrorStack> {
        let r = gen_random(&self.q)?;
        let c = self.helper(&x, &r)?;
        Ok((c, r))
    }

    pub fn mult_commitments(&mut self, cm: &[&BigNum]) -> Result<BigNum, ErrorStack> {
        // Multiply arry of commitments cm
        let res = cm.iter().fold(BigNum::from_u32(1)?, |acc, x| &acc * *x);
        let mut tmp = BigNum::new()?;
        tmp.nnmod(&res, &self.q, &mut self.ctx)?;
        Ok(tmp)
    }

    pub fn open(&mut self, c: &BigNum, x: &BigNum, args: &[&BigNum]) -> Result<bool, ErrorStack> {
        // c: commitment
        // x: the secret
        // args: array of randomness
        let total = args.iter().fold(BigNum::new()?, |acc, x| &acc + *x);
        let res = self.helper(&x, &total)?;
        Ok(&res == c)
    }    
}


#[test]
fn test_additive_secret_sharing() {

    let mut ss = CommitedAdditiveSecretSharing::new(512, 10).unwrap();
    let msg1 = 12;
    let shares1 = ss.share(msg1);
    for i in 0..10{
        let answer = ss.open(&shares1.commitments[i], &shares1.shares[i], &[&shares1.randomness[i]]);
        // println!("{}\n{}\n{}\n", , shares.commitments[i], shares.shares[i]);
        assert_eq!(answer.unwrap(), true);
    }  
    assert_eq!(ss.reconstruct(shares1.shares), BigNum::from_u32(msg1).unwrap());
    

}

