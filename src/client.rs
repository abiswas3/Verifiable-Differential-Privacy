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
        
        let (com, r) = self.commit(&last_share, ctx).unwrap(); 
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
    pub fn open(&self, c: &BigNum, x: &BigNum, r: &BigNum, ctx: &mut BigNumContext) -> Result<bool, ErrorStack> {
        // c: commitment
        // x: the secret
        // r: array of randomness
        
        let res = self.helper(&x, &r, ctx)?;    
        Ok(&res == c)
    }       

}

#[test]
pub fn test_voting(){

    // Parameters 
    let security_parameter = 4; // number bits of security to use
    let num_candidates = 5; // M
    let num_shares = 4; // K
    

    let mut public_param = crate::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap(); 
    let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);

    let vote = 0;    
    let share_of_shares = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers
    for dim in 0..num_candidates{
        let mut recons_share = BigNum::new().unwrap();
        for server_idx in 0..num_shares{
            recons_share = (&recons_share + &share_of_shares[dim].shares[server_idx]).rem(&client.q);
        }
        assert_eq!(vote as usize == dim, recons_share == BigNum::from_u32(1).unwrap());
    }


}

#[test]
#[should_panic]
pub fn test_bad_input1(){
    // Parameters 
    let security_parameter = 4; // number bits of security to use
    let num_candidates = 5; // M
    let num_shares = 4; // K

    let mut public_param = crate::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap(); 
    let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);

    // Should crash : Client cannot vote for a candidate not in 
    let vote = num_candidates;    
    let _ = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers
}

#[test]
pub fn test_commitments(){
    // Parameters 
    let security_parameter = 4; // number bits of security to use
    let num_candidates = 5; // M
    let num_shares = 4; // K

    let mut public_param = crate::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap(); 
    let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);

    // Should crash : Client cannot vote for a candidate not in 
    let vote = 0;    
    let share_of_shares = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers

    for dim in 0..num_candidates{
        for server_idx in 0..num_shares{
            let shr = &share_of_shares[dim].shares[server_idx];
            let rnd = &share_of_shares[dim].randomness[server_idx];
            let com = &share_of_shares[dim].commitments[server_idx];

            assert_eq!(true, client.open(&com, &shr, &rnd, &mut public_param.ctx).unwrap());
        }
    }
    
}

#[test]
#[should_panic]
pub fn test_bad_commitments(){
    // Parameters 
    let security_parameter = 4; // number bits of security to use
    let num_candidates = 5; // M
    let num_shares = 4; // K

    let mut public_param = crate::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap(); 
    let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);

    // Should crash : Client cannot vote for a candidate not in 
    let vote = 0;    
    let share_of_shares = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers

    for dim in 0..num_candidates{
        for server_idx in 0..num_shares{
            let shr = &share_of_shares[dim].shares[server_idx] + &BigNum::from_u32(1).unwrap();
            let rnd = &share_of_shares[dim].randomness[server_idx];
            let com = &share_of_shares[dim].commitments[server_idx];

            assert_eq!(true, client.open(&com, &shr, &rnd, &mut public_param.ctx).unwrap());
        }
    }
    
}