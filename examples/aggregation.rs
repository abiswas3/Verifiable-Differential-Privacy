use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;
use std::time::{Instant};

fn main(){

    let mut csprng = OsRng;
    let n_b = 262144;
    let n = 1000000;

    println!("Creating {} + {} = {} random 256 bit integers",n, n_b, (n + n_b));
    let mut inputs : Vec<Scalar> = Vec::new();
    for _ in 0..(n + n_b){
        let y: Scalar = Scalar::random(&mut csprng);
        inputs.push(y);
    }

    let mut answer = Scalar::zero();
    let now = Instant::now();
    for i in 0..(n + n_b){
        answer = answer + inputs[i];
    }
    let end = now.elapsed().as_millis();
    println!("Time Taken to add {} + {} = {} integers; {}ms", n, n_b, (n + n_b), end);
}