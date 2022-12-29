// extern crate rand_core;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use std::time::{Instant};


#[derive(Clone)]
pub struct Commitment {
    g: RistrettoPoint,
    h: RistrettoPoint,
}

impl Commitment {

    pub fn new(g: RistrettoPoint, h: RistrettoPoint)->Commitment{
        return Self{g, h};
    }

    pub fn commit(&self, message: &[u8; 32], r: Scalar) -> RistrettoPoint {

        let message_copy: [u8;32] = message.clone();
        // let message_copy = message.as_bytes().to_vec() as [u8; 32];
        let m = Scalar::from_canonical_bytes(message_copy).unwrap();
        let now = Instant::now();        
        let gm = &m * &self.g;
        let end = now.elapsed().as_micros();
        println!("ECC exp: {},", end);    
        let now = Instant::now();        
        let hr = &r * &self.h;
        let end = now.elapsed().as_micros();
        println!("ECC exp: {},", end);
    
        let ans = gm + hr;
        return ans
    }    

    pub fn open(&self, com: RistrettoPoint, message: &[u8; 32], r: Scalar)->bool{
        return self.commit(message, r) == com;
    }

    pub fn get_public_params(&self)->(RistrettoPoint, RistrettoPoint){
        return (self.g, self.h); 
    }
}


