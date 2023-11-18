use crate::generic_commitments::CurveCommitment;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
// use num_integer::Roots;
use crate::sigma_ff::ProofScalar;
// use crate::generic_commitments::CurveCommitment;
use sha3::{Digest, Sha3_256};
use rand_core::OsRng;
use coinflip::flip;

pub struct Board {
    pub g: RistrettoPoint,
    pub h: RistrettoPoint,
    pub com: CurveCommitment

}

impl Board {

    pub fn new(g: RistrettoPoint, h: RistrettoPoint)->Board{

        let com = CurveCommitment{g, h};    
        Self {g: g, h: h, com: com} 
    }

    pub fn binary_to_exp(&self, commitments: Vec<RistrettoPoint>)->RistrettoPoint{

        let n = commitments.len();
        // let r_zero = Scalar::zero();
        let base: i32 = 2; // an explicit type is required

        for i in 0..n{
            // let mut tmp_bytes: [u8; 32] = [0; 32];            
            let tmp: u128 = base.pow(i as u32) as u128;
            println!("{}", tmp);
        }
        return self.g;
    }

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

pub struct Client{
    pub num_shares: usize,
    pub g: RistrettoPoint,
    pub h: RistrettoPoint,
    pub com: CurveCommitment
}

impl Client{

    pub fn new(num_shares: usize, g: RistrettoPoint, h: RistrettoPoint)->Client{

        let com = CurveCommitment{g, h};    
        Self {num_shares: num_shares, g: g, h: h, com: com} 
    }

    pub fn send_input_to_sever(&self)->(Scalar, Scalar){

        // This should be 1 or 0 but it doesn't matter for this script
        let x = Scalar::one();
        let mut csprng = OsRng;
        let r: Scalar = Scalar::random(&mut csprng);
        return (x, r);
    }
}
pub struct Server{
    pub num_shares: usize,
    pub g: RistrettoPoint,
    pub h: RistrettoPoint,
    pub com: CurveCommitment,
    openings: Vec<Vec<(Scalar, Scalar)>>
}

pub struct DistBernoulliProof{
    pub or_proofs: Vec<ProofScalar>,
    pub aggregate: Scalar
}


impl Server{

    pub fn new(num_shares: usize, g: RistrettoPoint, h: RistrettoPoint)->Server{

        let com = CurveCommitment{g, h};    
        Self { num_shares: num_shares, g: g, h: h, com: com, openings:Vec::new() } 
    }

    pub fn get_opening(&self, opening_idx: usize, challenge_idx:usize)->u8{

        let (tmp, _) = self.openings[opening_idx][challenge_idx];
        if tmp == Scalar::one(){
            return  1;
        }
        else{
            return 0;        
        }
    }

    pub fn clear_openings(&mut self){
        self.openings = Vec::new();
    }

    pub fn get_random_value(&self)->Scalar{

        let mut csprng = OsRng;
        let r: Scalar = Scalar::random(&mut csprng);
        return r;
    }

    pub fn distributional_geometric_com(&mut self, precision_bits: usize, l: usize, k:usize)->Vec<DistBernoulliProof>{

        let mut coin_coms = Vec::new();
        for _ in 0..precision_bits{
            // do the computation baed on l and k and do this.
            coin_coms.push(self.distributional_commitment_bernoulli(l, k));
        }
        return coin_coms;        
    }

    pub fn distributional_commitment_bernoulli(&mut self, l: usize, k:usize)->DistBernoulliProof{

        let mut private_openings: Vec<(Scalar, Scalar)>= Vec::new();
        let mut proof_transcripts : Vec<ProofScalar> = Vec::new();    
        let mut count_ones = 0;    
        let mut count_zeroes = 0;
        let mut aggregate_rand: Scalar = Scalar::zero();
        for _ in 0..k{    
            // Prover commits to a bit
            let r = self.get_random_value();

            let b;    
            // All ones done
            if count_ones == l{
                b = false;
            }
            // All zeroes done
            else if count_zeroes == k-l {
                b = true;
            }
            // Flip uniformm
            else{
                b = flip();
            }
            
            let transcript;            
            if b{
                transcript = self.com.create_proof_1(r);
                private_openings.push((Scalar::one(), r));
                count_ones +=1;
            }
            else{
                transcript = self.com.create_proof_0(r);
                private_openings.push((Scalar::zero(), r));
                count_zeroes +=1;
            }

            aggregate_rand += r;            
            proof_transcripts.push(transcript);
        }

    self.openings.push(private_openings);
    return  DistBernoulliProof{or_proofs: proof_transcripts, aggregate: aggregate_rand};
            
    }
    
}