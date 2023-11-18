use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
// use rand::{rngs::StdRng, RngCore, SeedableRng};
use rand_core::OsRng;
use sha3::{Digest, Sha3_256};
use crate::sigma_ff::ProofScalar;
// use crate::converters::u32_to_bytes;

pub trait Commitment<X, Y>{

    fn sample_randomness(&self)->X;
    fn commit(&self, message: X, randomness: X)->Y;
    fn open(&self, message: X, randomness: X, com: Y)->bool;
}

pub trait SecretSharing{
    fn share(&self, message: &[u8; 32], num_shares: usize)->Vec<[u8;32]>;
    fn reconstruct(&self, shares: &Vec<[u8; 32]>, num_shares: usize)->[u8; 32];
}

pub struct CurveCommitment {
    pub g: RistrettoPoint,
    pub h: RistrettoPoint,
}

impl CurveCommitment{
    pub fn new(g: RistrettoPoint, h: RistrettoPoint)->CurveCommitment{
        return Self{g, h};
    }   

    pub fn create_proof_0(&self, rand: Scalar)->ProofScalar{

        // create FIAT shamir proof for when the secret is 0
        // let mut hasher = Sha3_256::new();
        let com = self.commit(Scalar::zero(), rand);
        let v1 = self.sample_randomness();
        let e1 = self.sample_randomness();
        let b = self.sample_randomness();

        // d1 : Cheat
        let ce1 = -(&com * &e1); // 1/c^{e1}
        let ge1 = &self.g * &e1; // g^{e1}
        let hv1 = &self.h * &v1;      
        let d1 = hv1 + ce1 + ge1; //h^{v1} x 1/c^{e1} x g^{e1}

        // d0 : Honest
        let d0 = &b * &self.h; // h^{b}  

        let mut hasher = Sha3_256::new();
        let mut input_to_rom: Vec<u8> = Vec::new();
        input_to_rom.extend(com.compress().as_bytes());        
        input_to_rom.extend(d0.compress().as_bytes());
        input_to_rom.extend(d1.compress().as_bytes());
        hasher.update(input_to_rom); // this will take d0, d1, commitment as a byte array
        let result: [u8;32] =  hasher.finalize().into();        
        let e = Scalar::from_bytes_mod_order(result); // In the interactive version this would come in round 2
        
        let e0 = e - e1;         
        let v0 = b + e0*rand;

        return ProofScalar{com, e0, e1, e, v0, v1, d0, d1};

    }
    

    pub fn create_proof_1(&self, rand: Scalar)->ProofScalar{

        // create FIAT shamir proof for when the secret is 0
        // let mut hasher = Sha3_256::new();
        let com = self.commit(Scalar::one(), rand);

        let v0 = self.sample_randomness();
        let e0 = self.sample_randomness();
        let b = self.sample_randomness();

        // d0 : Cheat
        let ce0 = -(&com * &e0); // 1/c^{e0}
        let hv0 = &self.h * &v0; // h^{0}     
        let d0 = hv0 + ce0; //h^{v0} x 1/c^{e0} 

        // d1 : Honest
        let d1 = &b * &self.h; // h^{b}  

        let mut hasher = Sha3_256::new();
        let mut input_to_rom: Vec<u8> = Vec::new();
        input_to_rom.extend(com.compress().as_bytes());        
        input_to_rom.extend(d0.compress().as_bytes());
        input_to_rom.extend(d1.compress().as_bytes());
        hasher.update(input_to_rom); // this will take d0, d1, commitment as a byte array
        let result: [u8;32] =  hasher.finalize().into();        
        let e = Scalar::from_bytes_mod_order(result); // In the interactive version this would come in round 2
        
        let e1 = e - e0;         
        let v1 = b + e1*rand;

        return ProofScalar{com, e0, e1, e, v0, v1, d0, d1};

    }

}

impl Commitment<Scalar, RistrettoPoint> for CurveCommitment{

    fn sample_randomness(&self)->Scalar {
        
        let mut csprng = OsRng;
        return Scalar::random(&mut csprng);

    }
    fn commit(&self, message: Scalar, randomness: Scalar)->RistrettoPoint {
                
        let gm = &message * &self.g;
        let hr = &randomness * &self.h;
    
        let ans = gm + hr;
        return ans
    }

    fn open(&self, message: Scalar, randomness: Scalar, com: RistrettoPoint)->bool {
        return self.commit(message, randomness) == com;
    }

    
}

