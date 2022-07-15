extern crate dp_client as ss;
use ss::server::Server;
// use std::ops::Rem;
// use openssl::bn::{BigNum};
use rand::Rng;

fn generate_random_vote(num_candidates: u32)->u32{

    let mut rng = rand::thread_rng();

    return rng.gen_range(0..num_candidates);
}
fn main(){

    

    // Parameters 
    let security_parameter = 16;
    let num_candidates = 512; // FIX this later: with an encode function
    let num_shares = 10; // num_servers
    let num_clients = 100;

    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    
    // let mut ss = ss::committed_additive::CommitedAdditiveSecretSharing::new(security_parameter, num_shares).unwrap();
    // println!("{}\n\n", client);
    
    // CREATING SERVERS
    let mut agg = Vec::new();
    for _ in 0..num_shares{
        agg.push(Server::new(num_shares, &public_param.p, &public_param.q, &public_param.g, &public_param.h));
    }
    
    
    let mut sum_of_inputs = 0;
    for _ in 0..num_clients{
        let mut client = ss::client::Client::new(num_shares, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let msg = generate_random_vote(num_candidates);
        sum_of_inputs += msg;
        let shares = client.share(msg, &mut public_param.ctx);

        
        for i in 0..num_shares{
            if i == 0{
                println!("{}", agg[i]);
            }
            // let tmp = ss.helper(&shares.shares[i], &shares.randomness[i]).unwrap();
            // println!("CLIENT Commitment: {} Calc: {}^{}*{}^{}={}", shares.commitments[i], ss.g, shares.shares[i], ss.h, shares.randomness[i], tmp);
            // let answer = ss.open(&shares.commitments[i], &shares.shares[i], &[&shares.randomness[i]]);
            // assert_eq!(answer.unwrap(), true);            
            agg[i].receive_share(&shares.shares[i], &shares.randomness[i], &shares.commitments[i], &mut public_param.ctx);
        }  
        break;
    }
    println!("Answer: {} Reconstruction {}", sum_of_inputs, 1);


}