use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
use std::ops::Rem;
use crate::utils::{gen_random, mod_exp};
use sha3::{Digest, Sha3_256};
use rand::Rng;

pub struct Client{
    num_servers: usize,
    num_candidates: u32,
    p: BigNum,
    q: BigNum,
    g: BigNum,
    h: BigNum,
    pub kappa: BigNum    
}
pub struct Share{
    pub commitments: Vec<BigNum>,
    pub randomness: Vec<BigNum>,
    pub shares: Vec<BigNum>,
}

impl Client{

    pub fn new(num_servers: usize, num_candidates: u32, _p: &BigNum, _q: &BigNum, _g: &BigNum, _h: &BigNum) -> Client {
       
        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
        
        let kappa = gen_random(_q).unwrap();
        Self{num_servers, num_candidates, p, q, g, h, kappa}
    }

    pub fn generate_random_vote(&self, num_candidates: u32)->u32{

        let mut rng = rand::thread_rng();
    
        return rng.gen_range(0..num_candidates);
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

    pub fn share(&self, _secret: &BigNum, ctx: &mut BigNumContext)->Share{

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
        
        let secret = &BigNum::new().unwrap() + _secret;
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


    pub fn create_cds94_proof_for_1(&self, share_1: &Share, ctx: &mut BigNumContext)->(BigNum, BigNum, BigNum, BigNum, BigNum, BigNum, BigNum){

        let mut hasher = Sha3_256::new();

        // let share_1 = self.share(_secret, ctx);
        let mut recons_share = BigNum::new().unwrap();
        let mut recons_rand = BigNum::new().unwrap();
        let mut recons_com = BigNum::from_u32(1).unwrap();

        for server_idx in 0..self.num_servers{
            recons_share = &(&recons_share + &share_1.shares[server_idx]) % &self.q;
            recons_rand = &(&recons_rand + &share_1.randomness[server_idx]) % &self.q;
            recons_com = &(&recons_com * &share_1.commitments[server_idx])% &self.p;
        }

        let v0 = gen_random(&self.q).unwrap();
        let e0 = gen_random(&self.q).unwrap();
        let b = gen_random(&self.q).unwrap();

        // Messages to send to the verifier
        let mut d1 = BigNum::new().unwrap();    
        let mut d0 = BigNum::new().unwrap();

        // d1
        _ = d1.mod_exp(&self.h, &b, &self.p, ctx); // h^{b}

        // d0
        _ = d0.mod_exp(&self.h, &v0, &self.p, ctx); // h^{v0}
        let mut tmp = BigNum::new().unwrap();
        let mut tmp2 = BigNum::new().unwrap();
        _ = tmp.mod_exp(&recons_com, &e0, &self.p, ctx); // c^{e0}
        _ = tmp2.mod_inverse(&tmp, &self.p, ctx); // 1/c^{e0}
        d0 = &(&d0 * &tmp2)% &self.p; //h^{v0} x 1/c^{e0}


        // --- ROUND 2 ------
        // let e = ss::utils::gen_random(&public_param.q).unwrap();

        // This will be fixed 256 bit security btw -- you'd have to extend with two challenges to get 512 bits of security.
        let mut input_to_rom = (&BigNum::new().unwrap() + &recons_com).to_vec();
        input_to_rom.append(&mut d0.to_vec());
        input_to_rom.append(&mut d1.to_vec());

        hasher.update(input_to_rom); // this will take d0, d1, commitment as a byte array
        let result = hasher.finalize();
        let e = BigNum::from_slice(&result).unwrap().rem(&self.q);

        // --- ROUND 3 ------
        let mut e1 = BigNum::new().unwrap();
        _ = e1.mod_sub(&e, &e0, &self.q, ctx);         
        let v1 = (&b + &(&e1*&recons_rand)).rem(&self.q);


        return (e0, e1, e, v0, v1, d0, d1);
        
    }

    pub fn create_cds94_proof_for_0(&self, share_1: &Share, ctx: &mut BigNumContext)->(BigNum, BigNum, BigNum, BigNum, BigNum, BigNum, BigNum){

        let mut hasher = Sha3_256::new();

        let mut recons_share = BigNum::new().unwrap();
        let mut recons_rand = BigNum::new().unwrap();
        let mut recons_com = BigNum::from_u32(1).unwrap();

        for server_idx in 0..self.num_servers{
            recons_share = &(&recons_share + &share_1.shares[server_idx]) % &self.q;
            recons_rand = &(&recons_rand + &share_1.randomness[server_idx]) % &self.q;
            recons_com = &(&recons_com * &share_1.commitments[server_idx])% &self.p;
        }

        let v1 = gen_random(&self.q).unwrap();
        let e1 = gen_random(&self.q).unwrap();
        let b = gen_random(&self.q).unwrap();

        // Messages to send to the verifier
        let mut d1 = BigNum::new().unwrap();    
        let mut d0 = BigNum::new().unwrap();

        // d0 : Honest
        _ = d0.mod_exp(&self.h, &b, &self.p, ctx); // h^{b}

        // d1 : Cheat       
        let mut tmp = BigNum::new().unwrap();
        let mut tmp2 = BigNum::new().unwrap();
        let mut tmp3 = BigNum::new().unwrap();        
        _ = d1.mod_exp(&self.h, &v1, &self.p, ctx); // h^{v1}
        _ = tmp.mod_exp(&recons_com, &e1, &self.p, ctx); // c^{e1}
        _ = tmp2.mod_inverse(&tmp, &self.p, ctx); // 1/c^{e1}
        _ = tmp3.mod_exp(&self.g, &e1, &self.p, ctx); // g^{e1}
        d1 = (&(&d1 * &tmp2) * &tmp3).rem(&self.p); //h^{v1} x 1/c^{e1} x g^{e1}


        // Challenge from verifier or hash if in FS mode
        // This will be fixed 256 bit security btw -- you'd have to extend with two challenges to get 512 bits of security.
        let mut input_to_rom = (&BigNum::new().unwrap() + &recons_com).to_vec();
        input_to_rom.append(&mut d0.to_vec());
        input_to_rom.append(&mut d1.to_vec());
        hasher.update(input_to_rom); // this will take d0, d1, commitment as a byte array
        let result = hasher.finalize();
        let e = BigNum::from_slice(&result).unwrap().rem(&self.q);

        // Final messages
        let mut e0 = BigNum::new().unwrap();
        _ = e0.mod_sub(&e, &e1, &self.q, ctx);         
        let v0 = (&b + &(&e0*&recons_rand)).rem(&self.q);

        return (e0, e1, e, v0, v1, d0, d1);
        
    }


}