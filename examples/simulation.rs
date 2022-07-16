extern crate dp_client as ss;
use openssl::bn::BigNum;
// use std::ops::Rem;
// use openssl::bn::BigNum;
use ss::server::Server;
// use std::ops::Rem;
// use openssl::bn::{Bi&gNum};
use rand::Rng;
// use itertools::Itertools;

fn generate_random_vote(num_candidates: u32)->u32{

    let mut rng = rand::thread_rng();

    return rng.gen_range(0..num_candidates);
}
fn main(){

    // Parameters 
    let security_parameter = 256;
    let num_candidates = 1; // Singe dim bin mean estimation for now
    let num_shares = 5; // num_servers
    let num_clients = 10000;

    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    // println!("{}\n\n", public_param);
    
    // CREATING SERVERS
    let mut agg = Vec::new();
    for _ in 0..num_shares{
        agg.push(Server::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h));
    }
    

    let mut truth = Vec::new();
    for _ in 0..num_candidates as usize{
        truth.push(0);
    }
    // let mut sum_of_inputs = 0;
    for _ in 0..num_clients{
        let client = ss::client::Client::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let msg = generate_random_vote(num_candidates);
        truth[msg as usize] +=1;
        
        let share_of_shares = client.vote(msg, &mut public_param.ctx);
        
        // sum_of_inputs += msg;
        for (dim, shares) in share_of_shares.iter().enumerate(){        
            for server_idx in 0..num_shares{
                // Each server receives their share and commitment
                agg[server_idx].receive_share(dim, &shares.shares[server_idx], &shares.randomness[server_idx], &shares.commitments[server_idx], &mut public_param.ctx);
                
                // Each server also recerives commitments for 
                agg[server_idx].receive_commitments(dim,  &shares.commitments);
            }                  
        }
        
    }
    for i in 0..num_candidates as usize{
        println!("Total Votes for candidate: {} =  {}", i, truth[i]);
    }
    println!("====================================");
    for dim in 0..num_candidates as usize{
        for server_idx in 0..num_shares{
            let v = &BigNum::new().unwrap() + &agg[server_idx].agg_shares[dim];
            let r =  &agg[server_idx].agg_randomness[dim];            
            agg[0].receive_tally_broadcast(dim, server_idx, &v, r, &mut public_param.ctx);
            agg[0].aggregate(dim, v);
        }
        println!("Total Votes for candidate: {} =  {}", dim, agg[0].ans[dim]);
    }

}