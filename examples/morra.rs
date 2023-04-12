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
    // let n_b = 1000;

    let mut b_sum = 0;    
    let now = Instant::now();    
    for _ in 0..n_b{
        
        let x0 = Scalar::random(&mut csprng);
        let r0 = Scalar::random(&mut csprng);
        let _ = client.com.commit(x0, r0);

        // Receives this one from other party
        let x1 = Scalar::random(&mut csprng);
        let r1 = Scalar::random(&mut csprng);
        // For an assert
        let _ = client.com.commit(x1, r1);

        // Threshold
        if (x0 + x1).to_bytes()[0] %2  == 0{
            b_sum += 1;
        }
        else{
            b_sum += 0;
        }
    }
    let end = now.elapsed().as_millis();
    println!("Time Taken generate {} coins; {} ms", n_b, end);
    println!("{:?}", b_sum);
}