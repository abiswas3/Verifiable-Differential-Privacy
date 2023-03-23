use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::constants;
use rand_core::OsRng;
use::dp_client::generic_commitments::{Commitment};
extern crate dp_client as ss;
use std::time::{Instant};


fn main(){

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let com = ss::generic_commitments::CurveCommitment{g, h};    

    let mut csprng = OsRng;
    let msg: Scalar = Scalar::random(&mut csprng);
    let now = Instant::now();
    let r: Scalar = Scalar::random(&mut csprng);
    com.commit(msg, r);
    let end = now.elapsed().as_micros();
    println!("Time Taken To compute One commitment: {} mu(s)", end);
}