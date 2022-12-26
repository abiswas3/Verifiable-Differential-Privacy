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
    
    for i in 3..14{
        let base: i32 = 2; // an explicit type is required        
        let num_candidates = base.pow(i) as usize/num_parallel_cores;        
        let mut elapsed_time = 0;
        for _ in 0..num_candidates{

            let client = ss::verifiable_client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);    
            let choice = client.generate_random_vote(num_candidates as u32);   
            let vote = client.vote(choice, &mut public_param.ctx);

            let now = Instant::now();
            for i in 0..num_candidates{
    
                if i == choice as usize{
                    let (e0, e1, e, v0, v1, d0, d1) = client.create_cds94_proof_for_1(&vote[i as usize], &mut public_param.ctx);        
                    let board = ss::audit::Board::new(&vote[i as usize].commitments, 
                        &public_param.p,
                        &public_param.q,
                        &public_param.g,
                        &public_param.h,
                        &e0, 
                        &e1, 
                        &e, 
                        &v0, 
                        &v1, 
                        &d0, 
                        &d1, 
                        num_shares);                
                    board.verify(&mut public_param.ctx);                    
        
                }
                else{
                    let (e0, e1, e, v0, v1, d0, d1) = client.create_cds94_proof_for_0(&vote[i as usize], &mut public_param.ctx);    
                    let board = ss::audit::Board::new(&vote[i as usize].commitments, 
                        &public_param.p,
                        &public_param.q,
                        &public_param.g,
                        &public_param.h,
                        &e0, 
                        &e1, 
                        &e, 
                        &v0, 
                        &v1, 
                        &d0, 
                        &d1, 
                        num_shares);                
                    board.verify(&mut public_param.ctx);                    
        
                }
            }
            let end = now.elapsed().as_micros();
            elapsed_time += end;
        }
        println!("{}:{}\n", num_candidates*num_parallel_cores, elapsed_time);
    }
    
}