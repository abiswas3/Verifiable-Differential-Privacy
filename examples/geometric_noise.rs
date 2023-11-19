// use core::num;


use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants;
// use sha3::digest::typenum::Pow;
// use ss::generic_commitments::Commitment;
// use ss::participants::Board;
extern crate dp_client as ss;
use rand::Rng;
use ss::generic_commitments::Commitment;

use ss::consants::MGRAIN;

fn main() {

    // Public Paramters
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let base_prob = 0.5276334472589853;
    
    let  num_shares = 2;
    let mut server = ss::participants::Server::new(num_shares, g, h);
    let verifier = ss::participants::Board::new(g,h);

    let precision_bits = 12;
    
    for _ in 0..100{
        
        server.clear_openings();
        println!("Base Geometric Probability Distribution: {}", base_prob);
        let dist_geom_com = server.distributional_geometric_com(precision_bits, base_prob);

        // Verifier picks a challenge bit for each the precision bits, which is picked by p_j(base_prob)
        let mut verifier_challenge_indices = Vec::new();
        for _ in 0..precision_bits{
            let challenge_idx = rand::thread_rng().gen_range(0..MGRAIN);
            verifier_challenge_indices.push(challenge_idx);
        }
                
        let mut bit_coms = Vec::new();
        for i in 0..precision_bits{                        
            bit_coms.push(dist_geom_com[i].or_proofs[verifier_challenge_indices[i]].com);                    
        }        


        // for i in 0..precision_bits{
        //     server.get_opening(i, verifier_challenge_indices[i]);
        // }

        let geom_noise_com = verifier.binary_to_exp(bit_coms);    
        let (geom_x, geom_r) = server.geometric_opening(verifier_challenge_indices);
        assert_eq!(verifier.com.commit(geom_x, geom_r), geom_noise_com);
        println!("{:?}", geom_x.as_bytes());
        
    }
    

}