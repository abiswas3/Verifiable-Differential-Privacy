use openssl::bn::{BigNum, BigNumContext};
use std::fmt;
use crate::utils::{gen_random};

pub struct SimpleClient{
    num_servers: usize,
    num_candidates: u32,
    p: BigNum,
    q: BigNum,
    g: BigNum,
    h: BigNum,    
}
pub struct NonCommittedShare{
    pub shares: Vec<BigNum>,
}

impl fmt::Display for SimpleClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("p", &self.p)
            .field("q", &self.q)
            .field("g", &self.g)
            .field("h", &self.h)
            .finish()
    }
}

impl SimpleClient{

    pub fn new(num_servers: usize, num_candidates: u32, _p: &BigNum, _q: &BigNum, _g: &BigNum, _h: &BigNum) -> SimpleClient {
       
        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
        
        Self{num_servers, num_candidates, p, q, g, h}
    }

    pub fn vote(&self, vote: u32, ctx: &mut BigNumContext)->Vec::<NonCommittedShare>{

        if vote > self.num_candidates -1 {
            panic!("crash and burn");
        }

        let mut encoded_vote = Vec::<NonCommittedShare>::with_capacity(self.num_candidates as usize);        
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

    pub fn share(&self, _secret: u32, ctx: &mut BigNumContext)->NonCommittedShare{

        let mut shares = Vec::new();

        for _ in 1..(self.num_servers){            
            let tmp = gen_random(&self.q).unwrap();             
            shares.push(tmp);
        }
        
        let secret = BigNum::from_u32(_secret).unwrap();
        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut last_share = BigNum::new().unwrap();
        _ = last_share.mod_sub(&secret, &total, &self.q, ctx);        
        shares.push(last_share);

        return NonCommittedShare{shares};
    }


}

#[test]
pub fn test_voting(){
    use std::ops::Rem;

    // Parameters 
    let security_parameter = 4; // number bits of security to use
    let num_candidates = 5; // M
    let num_shares = 4; // K
    
    let mut public_param = crate::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap(); 
    let client = SimpleClient::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);

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

