extern crate dp_client as ss;
use std::time::{Instant};
// use std::collections::HashMap;
// use ss::utils::print_vec;


// #[derive(Serialize, Deserialize)]

fn main(){
    // Parameters 
    let security_parameter = 256;
    
    println!("Varying Shares");
    for num_shares in [2, 4, 8, 16]{        
        let num_candidates = 16; // Singe dim bin mean estimation for now     
        let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    
        let now = Instant::now();
        let client = ss::simple_client::SimpleClient::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        client.vote(0, &mut public_param.ctx);
        let tmp1 = now.elapsed().as_millis();

        let now = Instant::now();
        let client = ss::client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let vote = 0;    
        let _ = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers
        let tmp2 = now.elapsed().as_millis();
        print!("{}:[{}, {}], ", num_shares, tmp1, tmp2);
    }

    println!("\nVarying Number of candidates");
    for num_candidates in [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]{        
        let num_shares = 4; // Singe dim bin mean estimation for now     
        let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    
        let now = Instant::now();
        let client = ss::simple_client::SimpleClient::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        client.vote(0, &mut public_param.ctx);
        let tmp1 = now.elapsed().as_millis();

        let now = Instant::now();
        let client = ss::client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let vote = 0;    
        let _ = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers
        let tmp2 = now.elapsed().as_millis();
        print!("{}:[{}, {}], ", num_candidates, tmp1, tmp2);
    }
    println!();


        
}