extern crate dp_client as ss;
use std::time::{Instant};

fn main(){

    let security_parameter = 512;
    let num_shares = 4; // NOT USED IN THIS EXAMPLE
    let num_candidates = 2; // NOT USED IN THIS EXAMPLE
    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();    
    // println!("{}", public_param);

    let client = ss::verifiable_client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);        

    let now = Instant::now();                        
    let proofs = client.create_proof_1(&mut public_param.ctx); // Verifier never figures this out    
    let end = now.elapsed().as_millis();
    println!("Time Taken To Create Non-Interactive Proof: {} ms",end);

    let now = Instant::now();                        

    let board = ss::audit::Board::new(&proofs.com, 
            &public_param.p,
            &public_param.q,
            &public_param.g,
            &public_param.h,
            &proofs.e0,
            &proofs.e1,
            &proofs.e,
            &proofs.v0,
            &proofs.v1,
            &proofs.d0,
            &proofs.d1,
            num_shares);
            board.verify(&mut public_param.ctx);
    let end = now.elapsed().as_millis();
    print!("Time taken to verify proof: {} ms", end);

}