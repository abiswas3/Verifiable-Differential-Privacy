// use rand_core::{OsRng};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::{rngs::StdRng, RngCore, SeedableRng};
// use crate::utils::{print_array};

#[derive(Clone)]
pub struct Commitment {
    g: RistrettoPoint,
    h: RistrettoPoint,
}

impl Commitment {

    pub fn new(g: RistrettoPoint, h: RistrettoPoint)->Commitment{
        return Self{g, h};
    }

    pub fn get_random_byte_array(&self)->[u8;32]{

        let seed = [0u8; 32];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        return bytes;
    }

    // pub fn get_timing_stats(&self, num_trials: usize)->(u128, u128){

    //     // let mut csprng = OsRng;
    //     let mut time_exp: u128 = 0;
    //     let mut time_mult = 0;
    //     for _ in 0..num_trials{
    //         // let r: Scalar = Scalar::random(&mut csprng);

    //         let msg = self.get_random_byte_array();
            
    //         let m = Scalar::from_bytes_mod_order(msg);
    //         let now = Instant::now();        
    //         let gm = &m * &self.g;
    //         let end = now.elapsed().as_micros();            
    //         time_exp += end;

    //         let now = Instant::now();        
    //         let _ = &gm + &self.g;
    //         let end = now.elapsed().as_micros();            
    //         time_mult += end;

    //     }

    //     return (time_exp/(num_trials as u128), time_mult/(num_trials as u128));
    // }

    pub fn commit(&self, message: &[u8; 32], r: Scalar) -> RistrettoPoint {

        let message_copy: [u8;32] = message.clone();
        // let message_copy = message.as_bytes().to_vec() as [u8; 32];
        let m = Scalar::from_canonical_bytes(message_copy).unwrap();        
        
        let gm = &m * &self.g;
        let hr = &r * &self.h;
    
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


