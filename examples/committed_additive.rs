extern crate dp_client as ss;
// use ss::server::Server;
// use std::ops::Rem;
// use openssl::bn::{BigNum};
// use rand::Rng;

// fn generate_random_vote(num_candidates: u32)->u32{

//     let mut rng = rand::thread_rng();

//     return rng.gen_range(0..num_candidates);
// }
fn main(){
    // // Parameters 
    // let security_parameter = 512;
    // let num_candidates = 4; // FIX this later: with an encode function
    // let num_shares = 10; // num_servers
    // let num_clients = 100;

    // let mut ss = ss::committed_additive::CommitedAdditiveSecretSharing::new(security_parameter, num_shares).unwrap();

    // let mut agg = Vec::new();
    // for _ in 0..num_shares{
    //     agg.push(Server::new(num_shares, &ss.p, &ss.q, &ss.g, &ss.h));
    // }
    
    
    
    // let mut sum_of_inputs = 0;
    // for _ in 0..num_clients{
    //     let msg = generate_random_vote(num_candidates);
    //     sum_of_inputs += msg;
    //     let shares = ss.share(msg);

    //     // for i in 0..num_shares{
    //     //     let answer = ss.open(&shares.commitments[i], &shares.shares[i], &[&shares.randomness[i]]);
    //     //     assert_eq!(answer.unwrap(), true);
    //     // }  
    //     for k in 0..num_shares{
    //         agg[k].receive_share(&shares.shares[k], &shares.randomness[k], &shares.commitments[k]);            
    //     }   
            
    // }

    // println!("Answer: {} Reconstruction {}", sum_of_inputs, 1);


}