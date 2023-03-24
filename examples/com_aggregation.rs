use curve25519_dalek::constants;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;
use::dp_client::generic_commitments::{Commitment};
use dp_client as ss;
use std::time::{Instant};

fn main(){

    let mut csprng = OsRng;
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;   
    
    let num_shares = 2;
    let client = ss::participants::Client::new(num_shares, g, h);

    let n_b = 262144;
    let n = 1000000;

    let mut coms_to_inputs : Vec<RistrettoPoint> = Vec::new();    
    println!("Creating {} + {} = {} random commitments (points on the curve)",n, n_b, (n + n_b));
    for _ in 0..(n + n_b){
        let x = Scalar::random(&mut csprng);
        let r = Scalar::random(&mut csprng);
        let com = client.com.commit(x, r);
        coms_to_inputs.push(com);
    }

    println!("Starting to aggregate");
    let mut answer = coms_to_inputs[0];
    let now = Instant::now();    
    for i in 1..(n + n_b){
        answer = answer + coms_to_inputs[i];
    }    
    let end = now.elapsed().as_millis();
    println!("Time Taken to add {} + {} = {} integers; {} ms", n, n_b, (n + n_b), end);


}