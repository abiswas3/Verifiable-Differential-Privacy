use openssl::bn::{BigNum, BigNumContext};
use crate::utils::{gen_random};

pub struct Client{
    num_servers: usize,
    num_candidates: u32,
    q: BigNum,
    pub kappa: BigNum    
}
pub struct Share{
    pub shares: Vec<BigNum>,
}


impl Client{

    pub fn new(num_servers: usize, num_candidates: u32, _q: &BigNum) -> Client {
       
        
        let q = &BigNum::new().unwrap() + _q;
        
        let kappa = gen_random(_q).unwrap();
        Self{num_servers, num_candidates, q, kappa}
    }

    pub fn generate_fake_vote(&self)->u32{
        return 1;
    } 
        
    pub fn vote(&self, vote: u32, ctx: &mut BigNumContext)->Vec::<Share>{

        if vote > self.num_candidates -1 {
            panic!("crash and burn");
        }

        let mut encoded_vote = Vec::<Share>::with_capacity(self.num_candidates as usize);        
        for i in 0..self.num_candidates as usize{
            if i as u32 == vote{
                encoded_vote.push(self.share(&BigNum::from_u32(1).unwrap(), ctx));
            }
            else{
                encoded_vote.push(self.share(&BigNum::from_u32(0).unwrap(), ctx));
            }            
        }
        return encoded_vote;
    }

    pub fn poplar_vote(&self, vote: u32, ctx: &mut BigNumContext)->Vec::<Share>{

        if vote > self.num_candidates -1 {
            panic!("crash and burn");
        }

        let mut encoded_vote = Vec::<Share>::with_capacity(self.num_candidates as usize);        
        for i in 0..self.num_candidates as usize{
            if i as u32 == vote{
                encoded_vote.push(self.share(&self.kappa, ctx));
            }
            else{
                encoded_vote.push(self.share(&BigNum::from_u32(0).unwrap(), ctx));
            }            
        }
        return encoded_vote;

    }

    pub fn get_ab_for_poplar(&self, a: &BigNum, b: &BigNum, c: &BigNum, ctx: &mut BigNumContext)->(Share, Share){

        let mut ucase_a  = BigNum::new().unwrap(); // A = 0
        _ = ucase_a.mod_sub(&self.kappa, &(&BigNum::from_u32(2).unwrap()*a), &self.q, ctx); //A = kappa - 2a

        let mut ucase_b  = BigNum::new().unwrap();
        let tmp = &(&(a*a) + b) + c; // a^2 + b +c 
        _ = ucase_b.mod_sub(&tmp, &(a*&self.kappa), &self.q, ctx); // a^2 + b +c - a*kappa

        return (self.share(&ucase_a, ctx), self.share(&ucase_b, ctx));
        
    }

    pub fn share(&self, _secret: &BigNum, ctx: &mut BigNumContext)->Share{

        let mut shares = Vec::new();

        for _ in 1..(self.num_servers){            
            let tmp = gen_random(&self.q).unwrap(); 

            shares.push(tmp);
        }
        
        let secret = &BigNum::new().unwrap() + _secret;
        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut last_share = BigNum::new().unwrap();
        _ = last_share.mod_sub(&secret, &total, &self.q, ctx);
        
        shares.push(last_share);

        return Share{ 
            shares: shares
        };
    }



}