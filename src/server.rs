use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
// use rand::random;
use std::ops::Rem;
use std::fmt;

use crate::utils::{gen_random, mod_exp};
// use crate::utils::calculate_q;
pub struct Server{
    pub agg_shares: Vec<BigNum>,
    pub agg_randomness: Vec<BigNum>,    
    num_servers: usize,
    pub p: BigNum,
    pub q: BigNum,
    pub g: BigNum,
    pub h: BigNum,
    pub ans:Vec<BigNum>,
    commitments: Vec<Vec<Vec<BigNum>>>   // dxM X M : for a single dimension 
}

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
    pub fn new(num_servers: usize, num_candidates: u32, _p: &BigNum, _q: &BigNum, _g: &BigNum, _h: &BigNum) -> Server {
       
        let mut agg_shares = Vec::<BigNum>::with_capacity(num_candidates as usize);   
        let mut agg_randomness = Vec::<BigNum>::with_capacity(num_candidates as usize);   
        let mut ans = Vec::<BigNum>::with_capacity(num_candidates as usize);   
        let mut commitments = Vec::new();
        for _ in 0..num_candidates{
            agg_shares.push(BigNum::new().unwrap());
            agg_randomness.push(BigNum::new().unwrap());            
            ans.push(BigNum::new().unwrap());
            commitments.push(Vec::new());
        }

        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
        
       
        
        Self{agg_shares, agg_randomness, num_servers, p, q, g, h, commitments, ans}
    }
    pub fn verify(&mut self, broadcasted_messages: &[&BigNum])->u8{        
        //TODO: for now only include legal votes
        assert_eq!(broadcasted_messages.len(), self.num_servers);
        return 1;
    }

    pub fn receive_share(&mut self, dimension: usize, share: &BigNum, randomness: &BigNum, com: &BigNum, ctx: &mut BigNumContext){

        // Servers make sure clients are not misbehaving
        // let opened = self.open(com, share, &[randomness], ctx).unwrap();        
        let res = self.helper(share, &randomness, ctx).unwrap();
        assert_eq!(res, com + &BigNum::new().unwrap());

        self.agg_shares[dimension] = (&self.agg_shares[dimension] + share).rem(&self.q);
        self.agg_randomness[dimension] = (&self.agg_randomness[dimension] + randomness).rem(&self.q);             
    }

    pub fn receive_commitments(&mut self, dimension: usize, coms: &Vec<BigNum>){

        // For multi dimesion this has to be fixed (FIXME)

        // _ later used for client index
        let mut coms_copy = Vec::new();
        for (_, com) in coms.iter().enumerate(){            
            let tmp = com + &BigNum::new().unwrap();            
            coms_copy.push(tmp);
        }

        self.commitments[dimension].push(coms_copy);
    }

    pub fn receive_tally_broadcast(&self, dimension: usize, server_idx: usize, v: &BigNum, r: &BigNum, ctx: &mut BigNumContext)->bool{
        // This is the broadcast during the tallying stage
        let mut res = BigNum::from_u32(1).unwrap();
        
        for com in &self.commitments[dimension]{
            res = (&res * &com[server_idx]).rem(&self.p);                    
        }
        
        let ans = self.helper(v, r, ctx).unwrap();
        assert_eq!(ans, res);
        return ans == res;
    }

    pub fn aggregate(&mut self, dimension: usize, v: BigNum){

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
    // pub fn verify_tally(&mut self, committments: &[&BigNum], ctx: &mut BigNumContext)->bool{        
    //     let lhs = committments.iter().fold(BigNum::from_u32(1).unwrap(), |acc, x| &acc * *x).rem(&self.q);
    //     let rhs = helper(&self.g, &self.h, &self.q, &self.agg_shares, &self.agg_randomness, ctx);
    //     println!("{}, {}", lhs, rhs);
    //     return rhs == lhs;
    // }
}
