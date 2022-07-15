use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
// use rand::random;
use std::ops::Rem;
use std::fmt;
use crate::utils::{gen_random, mod_exp};
// use crate::utils::calculate_q;
pub struct Server{
    agg_shares: BigNum,
    agg_randomness: BigNum,
    num_clients: u32,
    num_servers: usize,
    pub p: BigNum,
    pub q: BigNum,
    pub g: BigNum,
    pub h: BigNum,    
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
    pub fn new(num_servers: usize, _p: &BigNum, _q: &BigNum, _g: &BigNum, _h: &BigNum) -> Server {
       
        let num_clients = 0;
        let agg_shares = BigNum::new().unwrap();
        let agg_randomness = BigNum::new().unwrap();

        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;
        

        Self{agg_shares, agg_randomness, num_clients, num_servers, p, q, g, h}
    }
    pub fn verify(&mut self, broadcasted_messages: &[&BigNum])->u8{        
        //TODO: for now only include legal votes
        assert_eq!(broadcasted_messages.len(), self.num_servers);
        self.num_clients = self.num_clients + 1;
        return 1;
    }

    pub fn receive_share(&mut self, share: &BigNum, randomness: &BigNum, com: &BigNum, ctx: &mut BigNumContext){

        // Servers make sure clients are not misbehaving
        let opened = self.open(com, share, &[randomness], ctx).unwrap();
        assert_eq!(opened, true);
        
        self.agg_shares = (&self.agg_shares + share).rem(&self.q);
        self.agg_randomness = (&self.agg_shares + randomness).rem(&self.q);             
    }


    fn helper(&mut self, x1: &BigNum, r: &BigNum, ctx: &mut BigNumContext) -> Result<BigNum, ErrorStack> {
        // returns g^x1h^r        
        let tmp3 = mod_exp(&self.g, x1, &self.q, ctx).rem(&self.q);
        let tmp4 = mod_exp(&self.h, r, &self.q, ctx).rem(&self.q);                
        return Ok((&(tmp3) * &(tmp4)).rem(&self.q));
        
    }  
    
    pub fn commit(&mut self, x: &BigNum,  ctx: &mut BigNumContext) -> Result<(BigNum, BigNum), ErrorStack> {
        let r = gen_random(&self.q).unwrap();
        let c = self.helper(&x, &r, ctx)?;
        Ok((c, r))
    }

    pub fn mult_commitments(&mut self, cm: &[&BigNum]) -> Result<BigNum, ErrorStack> {
        // Multiply arry of commitments cm
        let res = (cm.iter().fold(BigNum::from_u32(1)?, |acc, x| &acc * *x)).rem(&self.q);
        Ok(res)
    }

    pub fn open(&mut self, c: &BigNum, x: &BigNum, args: &[&BigNum], ctx: &mut BigNumContext) -> Result<bool, ErrorStack> {
        // c: commitment
        // x: the secret
        // args: array of randomness
        let total = args.iter().fold(BigNum::new()?, |acc, x| &acc + *x);
        // println!("Server: x: {}\nr: {}\n\n", x, total);
        let res = self.helper(&x, &total, ctx)?;
        // println!("Server: c: {}\nc_hat: {}\n\n", c, res);
        Ok(&res == c)
    }  
    // pub fn verify_tally(&mut self, committments: &[&BigNum], ctx: &mut BigNumContext)->bool{        
    //     let lhs = committments.iter().fold(BigNum::from_u32(1).unwrap(), |acc, x| &acc * *x).rem(&self.q);
    //     let rhs = helper(&self.g, &self.h, &self.q, &self.agg_shares, &self.agg_randomness, ctx);
    //     println!("{}, {}", lhs, rhs);
    //     return rhs == lhs;
    // }
}
