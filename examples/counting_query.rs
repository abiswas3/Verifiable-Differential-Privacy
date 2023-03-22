use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants;
extern crate dp_client as ss;
use std::time::{Instant};

fn main(){

    // Initialise client
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let coms = ss::generic_commitments::CurveCommitment{g, h};
    let num_shares = 2;
    let client = ss::client::Client{num_shares, coms};


    // let transcript = client.coms.create_proof_1();

    // _ = board.verify(&transcript);

    // let base: i32 = 2; // an explicit type is required      
    // let delta: f64 = 1.0/(base.pow(10) as f64);

    // for i in 12..30{
    //     let n_b = base.pow(i);
    //     let float_nb = n_b as f64;
    //     let epsilon = (2.0/(delta* float_nb) as f64).sqrt()*10.0; // Epsilon
    //     print!("{:.2}, {}\t\n", epsilon, n_b);
    // }

    let n_b = 2097152;
    let now = Instant::now();
    for _ in 0..n_b{
        let _ = client.coms.create_proof_1();
    }
    let end = now.elapsed().as_millis();
    print!(",\t{},\t{}", end, n_b);

}