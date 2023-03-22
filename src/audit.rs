use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use crate::sigma_ff::{ProofScalar};
use sha3::{Digest, Sha3_256};



pub struct Board {

    pub g: RistrettoPoint,
    pub h: RistrettoPoint

}

impl Board {

    pub fn verify(&self, transcript: &ProofScalar) -> bool {

        // CHECK the hash of the initial transcript is equal to e and then 
        let mut hasher = Sha3_256::new();
        let mut input_to_rom: Vec<u8> = Vec::new();
        input_to_rom.extend(transcript.com.compress().as_bytes());
        input_to_rom.extend(transcript.d0.compress().as_bytes());
        input_to_rom.extend(transcript.d1.compress().as_bytes());
        hasher.update(input_to_rom); // this will take d0, d1, commitment as a byte array
        let result: [u8;32] =  hasher.finalize().into();        
        let e = Scalar::from_bytes_mod_order(result);
        assert_eq!(e, transcript.e);

        // transcript.e = hash(d0,d1, com. )
        assert_eq!(transcript.e, transcript.e1 + transcript.e0); // CHECK e = e0 + e1

        let ce0 = &transcript.com * &transcript.e0; //c^{e0}
        let hv0 = self.h * &transcript.v0; //h^{v0}
        assert_eq!(&transcript.d0 + &ce0, hv0); //d0 c^{e0} = h^{v0}

        let ce1 = &transcript.com * &transcript.e1; // c^{e1}

        let ge1 = self.g * &transcript.e1; // g^{e1}
        let hv1 = self.h * &transcript.v1;// h^{v1}

        assert_eq!(&transcript.d1 + &ce1, &ge1 + &hv1); //d1 c^{e1} = g^{e1}h^{v1}

        return true;
    }
    
}
