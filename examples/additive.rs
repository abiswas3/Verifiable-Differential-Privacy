extern crate dp_client as ss;
use openssl::bn::{BigNum};


fn main(){

    let mut ss = ss::additive::AdditiveSecretSharing::new(512, 10).unwrap();

    let msg1 = 12;
    let shares = ss.share(msg1);
    for (i, share) in shares.iter().enumerate(){
        println!("{}: {}", i, share);
        println!();
    }

    let msg1_hat = ss.reconstruct(shares);
    println!("Original message: {}\nReconstructed message: {}", BigNum::from_u32(msg1).unwrap(), msg1_hat);
    
}