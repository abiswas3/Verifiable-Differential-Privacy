extern crate dp_client as ss;
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
    let num_parallel_cores = 8;
    
    for i in 1..14{
        let mut elapsed_time = 0;        
        let base: i32 = 2; // an explicit type is required        
        let num_candidates = base.pow(i) as usize * 1450/num_parallel_cores; // M   
        let client = ss::verifiable_client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        
        
        // THIS is a proof for M coins:: this should repeat n_b times
        let choice = client.generate_random_vote(num_candidates as u32);   // Same as bit string
        let now = Instant::now();        
        let _ = client.create_input_proof(choice, &mut public_param.ctx);
        let end = now.elapsed().as_micros();
        elapsed_time += end;    

        // Play morra: just generate public coins
        // for i in 0..num_candidates{
        //     let board = ss::audit::Board::new(&proofs[i as usize].coms, 
        //         &public_param.p,
        //         &public_param.q,
        //         &public_param.g,
        //         &public_param.h,
        //         &proofs[i].e0, 
        //         &proofs[i].e1, 
        //         &proofs[i].e, 
        //         &proofs[i].v0, 
        //         &proofs[i].v1, 
        //         &proofs[i].d0, 
        //         &proofs[i].d1, 
        //         num_shares);      

        //     board.verify(&mut public_param.ctx);     
        // }


            
        
        println!("{}:{},\n",   base.pow(i) as usize, elapsed_time);
    }
    
}