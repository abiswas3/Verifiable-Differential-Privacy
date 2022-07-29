
use openssl::bn::{BigNum, BigNumContext};

// use rand::distributions::{Bernoulli, Distribution};
use std::ops::Rem;
use std::fmt;

use crate::utils::{gen_random, mod_exp};
// use crate::utils::calculate_q;
pub struct SimpleServer{
    pub agg_shares: Vec<BigNum>, // Each index corresponds to a input dimension
    pub last_received_shares: Vec<BigNum>, // Each index corresponds to a input dimension
    num_servers: usize,
    num_candidates: usize,
    pub p: BigNum, // Diffie Hellman sub group G_q in Z_p*
    pub q: BigNum, // order of diffie Hellman group G
    pub g: BigNum, // generator for G
    pub h: BigNum, // random element of G such that it is hard to solve h = g^\alpha for random \alpha
    pub ans:Vec<BigNum>, // Store the final reconstructed histogram over n clients    
}

// NOTE: 
// https://stackoverflow.com/questions/27589054/what-is-the-correct-way-to-use-lifetimes-with-a-struct-in-rust
// Good discussion about lifetimes of references


impl fmt::Display for  SimpleServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Server")
            .field("p", &self.p)
            .field("q", &self.q)
            .field("g", &self.g)
            .field("h", &self.h)
            .finish()
    }
}
impl SimpleServer{
    pub fn new(num_servers:usize, num_candidates: usize, _p: &BigNum, _q: &BigNum, _g: &BigNum, _h: &BigNum) -> SimpleServer {
       
        let mut agg_shares = Vec::<BigNum>::with_capacity(num_candidates as usize);   
        let mut agg_randomness = Vec::<BigNum>::with_capacity(num_candidates as usize);   
        
        let mut last_received_randomness = Vec::new();
        let mut last_received_shares = Vec::new();

        let mut ans = Vec::<BigNum>::with_capacity(num_candidates as usize);   

        for _ in 0..num_candidates{
            last_received_shares.push(BigNum::new().unwrap());
            last_received_randomness.push(BigNum::new().unwrap());

            agg_shares.push(BigNum::new().unwrap());
            agg_randomness.push(BigNum::new().unwrap());            
            ans.push(BigNum::new().unwrap());

        }

        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
                
        Self{agg_shares, last_received_shares,  num_servers, num_candidates, p, q, g, h, ans}
    }

    pub fn generate_fresh_morra(&self)->Vec<u8>{

        let mut morra =  Vec::new();
        let two = BigNum::from_u32(2).unwrap(); 
        for _ in 0..self.num_candidates{
            let tmp = gen_random(&self.q).unwrap();
            if tmp <= &self.q / &two{
                morra.push(1 as u8);
            }
            else{
                morra.push(0 as u8);
            }                    
        }
        
        return morra;
    }

    pub fn generate_fresh_randomness(&self)->Vec<BigNum>{

        // Randomness to verify if an input is legal or not
        let mut r_vec = Vec::new();
        // For each dim get randomness that is shared by all servers
        for _ in 0..self.num_candidates{
            r_vec.push(gen_random(&self.q).unwrap());
        }
        return r_vec;
    }

    

    pub fn sketching_test(&self, z_i: &Vec<BigNum>, z_i_star: &Vec<BigNum>, ctx: &mut BigNumContext){

        // This is the polynomial test to make sure input is valid
        // Soundness probability 2/q

        // TODO: if this test fails I need to drop commitments and subtract aggregations
        assert_eq!(z_i.len(), self.num_servers);

        let z = (z_i.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q); 
        let z_star = (z_i_star.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q);
        
        assert_eq!(mod_exp(&z, &BigNum::from_u32(2).unwrap(), &self.q, ctx), z_star);
        
    }

    pub fn broadcast(&self, r_vec: &Vec<BigNum>, ctx: &mut BigNumContext)->(BigNum, BigNum){

        // Received input shares and commitments and now the server must broadcast a message
        // to verify if the client input is legal 
        let mut z_i = BigNum::from_u32(0).unwrap();
        let mut z_i_star = BigNum::from_u32(0).unwrap();
        // Dot product
        for i in 0..self.num_candidates{                
            z_i = &z_i + &(&r_vec[i] * &self.last_received_shares[i]).rem(&self.q);
            
            let r_i_square = mod_exp(&r_vec[i], &BigNum::from_u32(2).unwrap(), &self.q, ctx);
            z_i_star = &z_i_star + &( &r_i_square * &self.last_received_shares[i]).rem(&self.q);
        }
        return (z_i, z_i_star);
    }


    pub fn receive_share(&mut self, dimension: usize, share: &BigNum){
        self.last_received_shares[dimension] = share + &BigNum::new().unwrap();
        self.agg_shares[dimension] = (&self.agg_shares[dimension] + share).rem(&self.q);        
    }



    pub fn aggregate(&mut self, dimension: usize, v: BigNum){

        // Reconsturction for additive shares.
        self.ans[dimension] = (&self.ans[dimension] + &v).rem(&self.q);
    }

  
}

