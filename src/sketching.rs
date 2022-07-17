use openssl::bn::{BigNum, BigNumContext};
use std::ops::Rem;
use crate::utils::{gen_random, mod_exp};


pub struct VoteVerifier{
    num_servers: usize,
    pub randomness: Vec<BigNum>, // The common vector
    pub commitments: Vec<BigNum>, // commitments of all the shares for each dimension 
    pub message_z: Vec<BigNum>, 
    pub message_z_star: Vec<BigNum>, 
    pub message_randomness: Vec<BigNum>,
    pub p: BigNum,
    pub q: BigNum,
    pub g: BigNum,
    pub h: BigNum,    
}

impl VoteVerifier{

    pub fn accept_message(&mut self, z_i: &BigNum, z_i_star: &BigNum, t_i: &BigNum){
        // let copy_com = &BigNum::new().unwrap() + com;
        // self.commitments.push(copy_com);
        let copy_z = &BigNum::new().unwrap() + z_i;
        self.message_z.push(copy_z);

        let copy_z_star = &BigNum::new().unwrap() + z_i_star;
        self.message_z_star.push(copy_z_star);

        let copy_t = &BigNum::new().unwrap() + t_i;
        self.message_randomness.push(copy_t);        
    }

    pub fn sketch(&self, ctx: &mut BigNumContext)->bool{

        if self.commitments.len() != self.num_servers{
            return false;
        }
        else if self.message_z_star.len() != self.num_servers{
            return false;
        }
        else if self.message_z.len() != self.num_servers{
            return false;
        }                
 
        let z_star = (self.message_z_star.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q);
        let z = (self.message_z.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q);

        let two = BigNum::from_u32(2).unwrap();
        return mod_exp(&z, &two, &self.q, ctx) ==  z_star;
        
    }

    pub fn authenticate(&self, coms: &Vec<BigNum>){

        let z_star = (self.message_z_star.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q);
    }
}