// use core::num;

use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use curve25519_dalek::constants;
use ss::generic_commitments::Commitment;
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
    let el_as_scalar: Scalar = Scalar::from_bits(bytes);
    
    let  num_shares = 2;
    let mut server = ss::participants::Server::new(num_shares, g, h);
    let verifier = ss::participants::Board::new(g,h);

    
    // Generate random number in the range [0, 99]
    for opening_idx in 0..100{

        
        let dist_bernoulli_com = server.distributional_commitment_bernoulli(l, k);        
        
        // Verifier verifies if the bit is 0 or 1
        for i in 0..k{            
            _ = verifier.verify(&dist_bernoulli_com.or_proofs[i]);
        }
            
        // Verfier aggregates coms and checks
        let mut agg_com = dist_bernoulli_com.or_proofs[0].com;
        for i in 1..k{
            agg_com += dist_bernoulli_com.or_proofs[i].com;
        }        
        // Check if agg_com = Com(l, agg_r)
        assert_eq!(verifier.com.commit(el_as_scalar, dist_bernoulli_com.aggregate), agg_com);
    
        let challenge_idx = rand::thread_rng().gen_range(0..k);        
        let tmp = server.get_opening(opening_idx, challenge_idx);
        print!("{:?} ", tmp);
        
    }
    
    println!();
}