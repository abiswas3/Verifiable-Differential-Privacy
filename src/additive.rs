use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;

pub struct AdditiveSecretSharing {
    pub num_shares: usize,
    pub p: BigNum,
    pub q: BigNum,
    pub g: BigNum,
    pub h: BigNum,
    ctx: BigNumContext,    
}

impl AdditiveSecretSharing{

    pub fn new(security: i32, num_shares: usize) -> Result<AdditiveSecretSharing, ErrorStack> {
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


    pub fn share(&mut self, _secret: u32)->Vec<BigNum>{

        let mut shares = Vec::new();
        for _ in 1..(self.num_shares){            
            shares.push(gen_random(&self.q).unwrap());
            
        }
        
        let secret = BigNum::from_u32(_secret).unwrap();
        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut last_share = BigNum::new().unwrap();
        _ = last_share.mod_sub(&secret, &total, &self.q, &mut self.ctx);
        
        shares.push(last_share);
        return shares;
    }

    pub fn reconstruct(&mut self, shares: Vec<BigNum>)->BigNum{

        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut reconstructed_value = BigNum::new().unwrap();
        _ = reconstructed_value.nnmod(&total, &self.q, &mut self.ctx);
        return reconstructed_value;
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


#[test]
fn test_additive_secret_sharing() {
    let mut commitment = AdditiveSecretSharing::new(512, 10).unwrap();
    let mut msg1 = 12;
    let mut shares = commitment.share(msg1);
    let mut msg1_hat = commitment.reconstruct(shares);
    assert_eq!(msg1_hat, BigNum::from_u32(msg1).unwrap());

    msg1 = 10231;
    shares = commitment.share(msg1);
    msg1_hat = commitment.reconstruct(shares);
    assert_eq!(msg1_hat, BigNum::from_u32(msg1).unwrap());

}
