use curve25519_dalek::{ristretto::RistrettoPoint};
use curve25519_dalek::constants;
use ss::generic_commitments::Commitment;
use ss::sigma_ff::ProofScalar;
extern crate dp_client as ss;
use std::time::{Instant};

fn main(){

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let num_shares = 2;

    // Clients send input to servers while publicly committing to input
    let client = ss::participants::Client::new(num_shares, g, h);
    let verifier = ss::participants::Board{g, h};
    // let n_b = 262144;
    let n_b = 8;

    let mut proofs: Vec<ProofScalar> = Vec::new();
    for _ in 0..n_b{        
        let r = client.com.sample_randomness();
        proofs.push(client.com.create_proof_0(r));
    }    

    let now = Instant::now();
    for i in 0..n_b{                
        _ = verifier.verify(&proofs[i]);
    }   
    let end = now.elapsed();
    println!("Time taken to sequentially verify {} proofs {} ms", n_b, end.as_millis());
    println!("Time taken to sequentially verify {} proofs {} mu(s)", n_b, end.as_micros());
}