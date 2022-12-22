use openssl::bn::{BigNum, BigNumContext};
use core::num;
// use openssl::error::ErrorStack;
// use rand::distributions::{Bernoulli, Distribution};
use std::ops::Rem;
use std::fmt;
// use rand::Rng;
use crate::utils::{gen_random};


// use crate::utils::calculate_q;
pub struct Server{
    pub agg_shares: Vec<BigNum>, // Each index corresponds to a input dimension
    pub agg_randomness: Vec<BigNum>, // Each index corresponds to a input dimension    
    pub last_received_shares: Vec<BigNum>, // Each index corresponds to a input dimension
    pub last_received_randomness: Vec<BigNum>, // Each index corresponds to a input dimension    
    pub p: BigNum, // Diffie Hellman sub group G_q in Z_p*
    pub q: BigNum, // order of diffie Hellman group G
    pub g: BigNum, // generator for G
    pub h: BigNum, // random element of G such that it is hard to solve h = g^\alpha for random \alpha
    pub ans:Vec<BigNum>, // Store the final reconstructed histogram over n clients,
    pub num_candidates: usize,    
}

// NOTE: 
// https://stackoverflow.com/questions/27589054/what-is-the-correct-way-to-use-lifetimes-with-a-struct-in-rust
// Good discussion about lifetimes of references
pub struct Share{
    pub commitments: Vec<BigNum>,
    pub randomness: Vec<BigNum>,
    pub shares: Vec<BigNum>,
}

impl fmt::Display for  Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Server")
            .field("p", &self.p)
            .field("q", &self.q)
            .field("g", &self.g)
            .field("h", &self.h)
            .finish()
    }
}

impl Server{

    pub fn new(num_servers:usize, num_candidates: usize, _p: &BigNum, _q: &BigNum, _g: &BigNum, _h: &BigNum) -> Server {
       
        let mut agg_shares = Vec::<BigNum>::with_capacity(num_candidates as usize);   
        let mut agg_randomness = Vec::<BigNum>::with_capacity(num_candidates as usize);   
        
        let mut last_received_randomness = Vec::new();
        let mut last_received_shares = Vec::new();

        let mut ans = Vec::<BigNum>::with_capacity(num_candidates as usize);   

        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
                
        Self{agg_shares, agg_randomness, last_received_shares, last_received_randomness, p, q, g, h, ans, num_candidates}
    }

    pub fn multiply_first(&self, x_i: &BigNum, y_i: &BigNum, a_i: &BigNum, b_i: &BigNum, ctx: &mut BigNumContext)->(BigNum, BigNum){

        let mut d_i = BigNum::new().unwrap();
        let mut e_i = BigNum::new().unwrap();

        _ = d_i.mod_sub(x_i, a_i, &self.q, ctx);
        _ = e_i.mod_sub(y_i, b_i, &self.q, ctx);

        return (d_i, e_i);
    }
    pub fn multiply_second(&self, a_i: &BigNum, b_i: &BigNum, c_i: &BigNum, e: &BigNum, d: &BigNum)->BigNum{

        let db = d*b_i;
        let ea = e*a_i;
        
        
        let xy_i = &(&(&db + &ea) + c_i);
        return xy_i.rem(&self.q);

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

    pub fn create_sketch_share_one(&self, x: &Vec<BigNum>, randomness: &Vec<BigNum>, a_i: &BigNum, b_i: &BigNum, ctx: &mut BigNumContext)->(BigNum, BigNum){

        // let z = (z_i.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q); 
        // let z_star = (z_i_star.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q);
        let mut z = BigNum::new().unwrap();        
        let mut tmp = BigNum::new().unwrap();
        for i in 0..self.num_candidates{
            _ = tmp.mod_mul(&x[i], &randomness[i], &self.q, ctx);
            z = (&z + &tmp).rem(&self.q);
        }

        // I need to get a share of [z*z]_i from z_i
        return self.multiply_first(&z, &z, a_i, b_i, ctx);        
    }

    pub fn create_sketch_share_two(&self, x: &Vec<BigNum>, randomness: &Vec<BigNum>, a_i: &BigNum, b_i: &BigNum, c_i: &BigNum, e: &BigNum, d: &BigNum, ctx: &mut BigNumContext)->BigNum{

        let z_squared_share = self.multiply_second(a_i, b_i, c_i, e, d);

        let mut z_star = BigNum::new().unwrap();
        let mut tmp = BigNum::new().unwrap();        
        for i in 0..self.num_candidates{
            _ = tmp.mod_mul(&x[i], &(&randomness[i] * &randomness[i]), &self.q, ctx);
            z_star = (&z_star + &tmp).rem(&self.q);
        }
        
        _ = tmp.mod_sub(&z_squared_share, &z_star, &self.q, ctx);
        return tmp;        
    }

}

#[test]
fn test_sketching(){

    // use crate::utils::{gen_random, additive_share};
    use crate::public_parameters::PublicParams;
    use crate::prio::Server;
    use crate::verifiable_client::Client;
    use crate::beaver_triple::BeaverTriple;

    let security_parameter = 4;
    let num_candidates = 2; // Doesn't play a role in this test but need it to initialise server
    let num_shares = 3; // num_servers
    let mut public_param = PublicParams::new(security_parameter, num_shares).unwrap();
    println!("{}", public_param);

    let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);    
    let choice = client.generate_random_vote(num_candidates as u32);       
    let vote = client.vote(choice, &mut public_param.ctx);

    let mut servers = Vec::new();
    for _ in 0..num_shares{
        let server = Server::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        servers.push(server);
    }

    // Pre-processing
    let beaver_triples = BeaverTriple::new(num_shares, &public_param.q, &mut public_param.ctx);
    let randomness = servers[0].generate_fresh_randomness();
    // end

    // ROUND 1 of MPC
    let mut d = BigNum::new().unwrap();
    let mut e = BigNum::new().unwrap();    
    for server_idx in 0..num_shares{
        let mut shares = Vec::new(); // Get the shares for server i (copy it over)
        for coord in 0..num_candidates{
            shares.push(&vote[coord].shares[server_idx] + &BigNum::new().unwrap());
        }
        let (d_i, e_i) = servers[server_idx].create_sketch_share_one(&shares, &randomness, &beaver_triples.a_shares[server_idx], &beaver_triples.b_shares[server_idx], &mut public_param.ctx);
        d = (&d + &d_i).rem(&public_param.q);
        e = (&e + &e_i).rem(&public_param.q);
    }

    // ROUND 2 of MPC
    let mut ans = BigNum::new().unwrap();
    for server_idx in 0..num_shares{
        let mut shares = Vec::new(); // Get the shares for server i (copy it over)
        for coord in 0..num_candidates{
            shares.push(&vote[coord].shares[server_idx] + &BigNum::new().unwrap());
        }
        let tmp = servers[server_idx].create_sketch_share_two(&shares, &randomness, &beaver_triples.a_shares[server_idx], &beaver_triples.b_shares[server_idx], &beaver_triples.c_shares[server_idx], &e, &d, &mut public_param.ctx);
        ans = (&ans + &tmp).rem(&public_param.q);
    }    
    ans = (&ans +  &(&d*&e)).rem(&public_param.q);
    
    assert_eq!(ans, BigNum::new().unwrap());
}