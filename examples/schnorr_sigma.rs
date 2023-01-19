extern crate dp_client as ss;
use openssl::bn::{BigNum};
use std::env;
use std::time::{Instant};

fn main(){
    env::set_var("RUST_BACKTRACE", "1");

    // Modify this to do any bitstring

    // Parameters 
    let security_parameter = 256;
    // let num_candidates = 4; // Singe dim bin mean estimation for now
    let num_shares = 4; // num_servers    
    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    // println!("{}", public_param);
    // let num_parallel_cores = 8;
    
    for i in 1..14{

        let base: i32 = 2; // an explicit type is required        
        let num_candidates = base.pow(i) as usize;        // Number of dimensions I have to deal with

        let client = ss::verifiable_client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);    
        let choice = client.generate_random_vote(num_candidates as u32);   

        let proofs = client.create_input_proof(choice, &mut public_param.ctx); // A proof for each of the coordinates 

        // Go through each proof and verify it.
        let mut elapsed_time = 0;
        for i in 0..num_candidates{
            let board = ss::audit::Board::new(&proofs[i as usize].coms, 
                &public_param.p,
                &public_param.q,
                &public_param.g,
                &public_param.h,
                &proofs[i].e0, 
                &proofs[i].e1, 
                &proofs[i].e, 
                &proofs[i].v0, 
                &proofs[i].v1, 
                &proofs[i].d0, 
                &proofs[i].d1, 
                num_shares);      

            let now = Instant::now();
            board.verify(&mut public_param.ctx);     
            let end = now.elapsed().as_micros();
            elapsed_time += end;    
        }
        println!("{}:{},\n", num_candidates, elapsed_time);
    }
    
}