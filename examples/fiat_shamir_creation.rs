use curve25519_dalek::{ristretto::RistrettoPoint};
use curve25519_dalek::constants;
use ss::generic_commitments::Commitment;
extern crate dp_client as ss;
use std::time::{Instant};

fn main(){

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let num_shares = 2;

    // Clients send input to servers while publicly committing to input
    let client = ss::participants::Client::new(num_shares, g, h);

    for _ in 0..100{
        let now = Instant::now();
        let r = client.com.sample_randomness();
        let _ = client.com.create_proof_0(r);
        let end = now.elapsed().as_micros();
        println!("{}", end);
    }
    
    
}