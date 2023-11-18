// use core::num;

use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use curve25519_dalek::constants;
// use sha3::digest::typenum::Pow;
// use ss::generic_commitments::Commitment;
// use ss::participants::Board;
extern crate dp_client as ss;
use rand::Rng;

fn main() {

    // Public Paramters
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let k = 10;
    let l = 2;
    let mut bytes: [u8; 32] = [0; 32];
    bytes[0] = l as u8;
    // let el_as_scalar: Scalar = Scalar::from_bits(bytes);
    
    let  num_shares = 2;
    let mut server = ss::participants::Server::new(num_shares, g, h);
    let verifier = ss::participants::Board::new(g,h);

    let precision_bits = 12;
    
    // Generate random number in the range [0, 99]
    for _ in 0..1{
        
        let dist_geom_com = server.distributional_geometric_com(precision_bits, l, k);

        // Verifier needs verify all these commitments

        let mut verifier_challenge_indices = Vec::new();
        for _ in 0..precision_bits{
            let challenge_idx = rand::thread_rng().gen_range(0..k);
            verifier_challenge_indices.push(challenge_idx);
        }
                
        // let mut agg_com = dist_geom_com[0].or_proofs[verifier_challenge_indices[0]].com;
        let mut bit_coms = Vec::new();
        for i in 0..precision_bits{                        
            bit_coms.push(dist_geom_com[i].or_proofs[verifier_challenge_indices[i]].com);                    
        }        
        verifier.binary_to_exp(bit_coms);    
    }
    
    println!();
}