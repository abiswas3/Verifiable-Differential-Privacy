
use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
use std::ops::Rem;
use std::fmt;

use crate::utils::{gen_random, mod_exp};
// use crate::utils::calculate_q;
pub struct Server{
    pub agg_shares: Vec<BigNum>, // Each index corresponds to a input dimension
    pub agg_randomness: Vec<BigNum>, // Each index corresponds to a input dimension    

    pub last_received_shares: Vec<BigNum>, // Each index corresponds to a input dimension
    pub last_received_randomness: Vec<BigNum>, // Each index corresponds to a input dimension    
    num_servers: usize,
    num_candidates: usize,
    pub p: BigNum, // Diffie Hellman sub group G_q in Z_p*
    pub q: BigNum, // order of diffie Hellman group G
    pub g: BigNum, // generator for G
    pub h: BigNum, // random element of G such that it is hard to solve h = g^\alpha for random \alpha
    pub ans:Vec<BigNum>, // Store the final reconstructed histogram over n clients
    pub commitments: Vec<Vec<Vec<BigNum>>>   // d x n X M : Total commitments received by a single server
}

// NOTE: 
// https://stackoverflow.com/questions/27589054/what-is-the-correct-way-to-use-lifetimes-with-a-struct-in-rust
// Good discussion about lifetimes of references


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

        let mut commitments = Vec::new();
        for _ in 0..num_candidates{
            last_received_shares.push(BigNum::new().unwrap());
            last_received_randomness.push(BigNum::new().unwrap());

            agg_shares.push(BigNum::new().unwrap());
            agg_randomness.push(BigNum::new().unwrap());            
            ans.push(BigNum::new().unwrap());
            commitments.push(Vec::new());
        }

        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
                
        Self{agg_shares, agg_randomness, last_received_shares, last_received_randomness,  num_servers, num_candidates, p, q, g, h, commitments, ans}
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

    pub fn generate_noise_shares(&self){

        // TODO: DP stuff
    }
    
    pub fn verify_sketching_messages(&self, server_idx:usize, r_vec: &Vec<BigNum>, z: &BigNum, z_star: &BigNum, t: &BigNum, t_star: &BigNum, ctx: &mut BigNumContext){

        // This is the test to verify that the other server (given by server_idx) has sent 
        // the appropriate messages

        let last_client_idx = self.commitments[0].len() - 1;
        let mut commitments = Vec::new();
        for dim in 0..self.num_candidates{            
            let mut tmp = &BigNum::new().unwrap() + &self.commitments[dim][last_client_idx][server_idx];            
            tmp = mod_exp(&tmp, &r_vec[dim], &self.p, ctx);
            commitments.push(tmp);
        }
        let lhs = commitments.iter().fold(BigNum::from_u32(1).unwrap(), |acc, x| &acc * x).rem(&self.p);
        let rhs = self.helper(&z, &t, ctx).unwrap();
        assert_eq!(lhs, rhs);

        
        commitments = Vec::new();
        for dim in 0..self.num_candidates{            
            let mut tmp = &BigNum::new().unwrap() + &self.commitments[dim][last_client_idx][server_idx];            
            
            let r_squared = mod_exp(&r_vec[dim], &BigNum::from_u32(2).unwrap(), &self.q, ctx);

            tmp = mod_exp(&tmp, &r_squared, &self.p, ctx);
            commitments.push(tmp);
        }        
        let lhs = commitments.iter().fold(BigNum::from_u32(1).unwrap(), |acc, x| &acc * x).rem(&self.p);
        let rhs = self.helper(&z_star, &t_star, ctx).unwrap();        

        assert_eq!(lhs, rhs);
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

    pub fn broadcast(&self, r_vec: &Vec<BigNum>, ctx: &mut BigNumContext)->(BigNum, BigNum, BigNum, BigNum){

        // Received input shares and commitments and now the server must broadcast a message
        // to verify if the client input is legal 

        let mut z_i = BigNum::from_u32(0).unwrap();
        let mut z_i_star = BigNum::from_u32(0).unwrap();
        let mut t = BigNum::from_u32(0).unwrap();
        let mut t_star = BigNum::from_u32(0).unwrap();
        // Dot product
        for i in 0..self.num_candidates{                
            z_i = &z_i + &(&r_vec[i] * &self.last_received_shares[i]).rem(&self.q);
            t = &t + &(&r_vec[i] * &self.last_received_randomness[i]).rem(&self.q);            
            
            let r_i_square = mod_exp(&r_vec[i], &BigNum::from_u32(2).unwrap(), &self.q, ctx);
            z_i_star = &z_i_star + &( &r_i_square * &self.last_received_shares[i]).rem(&self.q);
            t_star = &t_star + &(&r_i_square * &self.last_received_randomness[i]).rem(&self.q);            
        }
        return (z_i, z_i_star, t, t_star);
    }


    pub fn receive_share(&mut self, dimension: usize, share: &BigNum, randomness: &BigNum, com: &BigNum, ctx: &mut BigNumContext){

        // Servers make sure clients are not misbehaving
        let res = self.helper(share, &randomness, ctx).unwrap();
        assert_eq!(res, com + &BigNum::new().unwrap());
    

        self.last_received_shares[dimension] = share + &BigNum::new().unwrap();
        self.last_received_randomness[dimension] = randomness + &BigNum::new().unwrap();

        self.agg_shares[dimension] = (&self.agg_shares[dimension] + share).rem(&self.q);
        self.agg_randomness[dimension] = (&self.agg_randomness[dimension] + randomness).rem(&self.q);             
    }

    pub fn receive_commitments(&mut self, dimension: usize, coms: &Vec<BigNum>){

        // _ later used for client index
        let mut coms_copy = Vec::new();
        for (_, com) in coms.iter().enumerate(){            
            let tmp = com + &BigNum::new().unwrap();            
            coms_copy.push(tmp);
        }

        self.commitments[dimension].push(coms_copy);
    }

    pub fn receive_tally_broadcast(&self, dimension: usize, server_idx: usize, v: &BigNum, r: &BigNum, ctx: &mut BigNumContext)->bool{
        
        // Received broadcasted messages for the tallying phase and ensures there has been no tampering

        let mut res = BigNum::from_u32(1).unwrap();
        
        for com in &self.commitments[dimension]{
            res = (&res * &com[server_idx]).rem(&self.p);                    
        }
        
        let ans = self.helper(v, r, ctx).unwrap();
        assert_eq!(ans, res);
        return ans == res;
    }

    pub fn aggregate(&mut self, dimension: usize, v: BigNum){

        // Reconsturction for additive shares.
        self.ans[dimension] = (&self.ans[dimension] + &v).rem(&self.q);
    }

    pub fn helper(& self, x1: &BigNum, r: &BigNum, ctx: &mut BigNumContext) -> Result<BigNum, ErrorStack> {
        // returns g^x1h^r        
        let tmp3 = mod_exp(&self.g, x1, &self.p, ctx);
        let tmp4 = mod_exp(&self.h, r, &self.p, ctx);                
        return Ok((&(tmp3) * &(tmp4)).rem(&self.p));        
    }  
    
    pub fn commit(&self, x: &BigNum,  ctx: &mut BigNumContext) -> Result<(BigNum, BigNum), ErrorStack> {
        let r = gen_random(&self.q).unwrap();
        let c = self.helper(&x, &r, ctx)?;
        Ok((c, r))
    }

    pub fn mult_commitments(&mut self, cm: &[&BigNum]) -> Result<BigNum, ErrorStack> {
        // Multiply arry of commitments cm
        let res = (cm.iter().fold(BigNum::from_u32(1)?, |acc, x| &acc * *x)).rem(&self.p);
        Ok(res)
    }    

    pub fn open(&self, c: &BigNum, x: &BigNum, args: &[&BigNum], ctx: &mut BigNumContext) -> Result<bool, ErrorStack> {
        // c: commitment
        // x: the secret
        // args: array of randomness
        let total = args.iter().fold(BigNum::new()?, |acc, x| &acc + *x);        
        let res = self.helper(&x, &total, ctx)?;
    
        Ok(&res == c)
    }  
}

// Need to write more tests for each function (TODO: This text needs to be fixed)
#[test]
fn test_agg_verify() {

    use crate::public_parameters::PublicParams;
    let security_parameter = 8;
    let num_candidates = 2; // Doesn't play a role in this test but need it to initialise server
    let num_shares = 10; // num_servers
    let mut public_param = PublicParams::new(security_parameter, num_shares).unwrap();

    // agg_verify(&self, z_i: Vec<BigNum>, z_i_star: Vec<BigNum>, ctx: &mut BigNumContext)->bool
    
    // Creating variables to hold the true answer
    let mut truth_z = BigNum::new().unwrap();
    let mut truth_z_star = BigNum::new().unwrap();

    // Simulating the different z_i and z_star each server sends out
    let mut z_i = Vec::new();
    let mut z_i_star = Vec::new();
    for _ in 0..num_shares{
        // Each server must generate this
        let tmp1 = gen_random(&public_param.q).unwrap();
        let tmp2 = gen_random(&public_param.q).unwrap();
        
        truth_z = (&truth_z + &tmp1).rem(&public_param.q);
        z_i.push(tmp1);        
        truth_z_star = (&truth_z_star + &tmp2).rem(&public_param.q);
        z_i_star.push(tmp2);
    }    
    
    // Call the aggregation function here (Need to add commitments)
    let server = Server::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
    _ = server.sketching_test(&z_i, &z_i_star, &mut public_param.ctx);

    
}
