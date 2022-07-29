extern crate dp_client as ss;
use std::time::{Instant};
// use std::collections::HashMap;
// use ss::utils::print_vec;
use std::io::Write;                                                                                                                                                                                                                                                                                                                    
use std::fs::File;    

// #[derive(Serialize, Deserialize)]

fn main(){
    // Parameters 
    let security_parameter = 256;
    let num_candidates = 5; // Singe dim bin mean estimation for now
    let num_shares = 4; // num_servers
    
    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();

    let mut coms = Vec::new();
    let mut no_coms = Vec::new();

    let epochs = 100;
    for _ in 0..epochs{
        let now = Instant::now();
        let client = ss::simple_client::SimpleClient::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        client.vote(0, &mut public_param.ctx);

        let tmp = now.elapsed().as_micros();
        no_coms.push(tmp);            
    }

    for _ in 0..epochs{
        let now = Instant::now();
        let client = ss::client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let vote = 0;    
        let _ = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers
        let tmp = now.elapsed().as_micros();
        coms.push(tmp);            
    }

    let mut f = File::create("coms.vtk").expect("Unable to create file");                                                                                                          
    for i in &coms{                                                                                                                                                                  
        write!(f, "{}, ", i).unwrap();        
    }
    
    let mut f = File::create("no_coms.vtk").expect("Unable to create file");                                                                                                          
    for i in &no_coms{                                                                                                                                                                  
        write!(f, "{}, ", i).unwrap();        
    }    
}