extern crate rand_core;

// use curve25519_dalek::constants;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
// use sha2::Sha512;



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
        let gm = &m * &self.g;
        let hr = &r * &self.h;
        return gm + hr;
    }    

    pub fn open(&self, com: RistrettoPoint, message: &[u8; 32], r: Scalar)->bool{
        return self.commit(message, r) == com;
    }

    pub fn get_public_params(&self)->(RistrettoPoint, RistrettoPoint){
        return (self.g, self.h); 
    }
}

pub struct Share{
    pub commitments: Vec<RistrettoPoint>,
    pub randomness: Vec<Scalar>,
    pub shares: Vec<Scalar>,
}

// impl Share{

//     pub fn new()->Share{

//         let commitments = Vev<RistrettoPoint>
//     }
// }
