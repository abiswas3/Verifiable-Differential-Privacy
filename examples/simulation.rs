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
    let security_parameter = 8;
    let num_candidates = 2; // Singe dim bin mean estimation for now
    let num_shares = 5; // num_servers
    let num_clients = 100;

    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();

    // GENERATOR DEBUG
    // let mut big_i = BigNum::new().unwrap();
    // let mut generator = Vec::new();
    // while big_i < public_param.q{        
    //     let mut tmp = BigNum::new().unwrap();
    //     _ = tmp.mod_exp(&public_param.g, &big_i, &public_param.p, &mut public_param.ctx);        
    //     generator.push(tmp);
    //     big_i = &big_i + &BigNum::from_u32(1).unwrap();
    // }
    // generator.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{}\n\n", public_param);
    // for (i, k) in generator.iter().enumerate(){
    //     println!(" {}:{}", i, k);
    // }
    
    // CREATING SERVERS
    let mut agg = Vec::new();
    for k in 0..num_shares{
        agg.push(Server::new(k, num_shares, &public_param.p, &public_param.q, &public_param.g, &public_param.h));
    }
    

    let mut sum_of_inputs = 0;
    for client_idx in 0..num_clients{
        let mut client = ss::client::Client::new(num_shares, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let msg = generate_random_vote(num_candidates);
        sum_of_inputs += msg;
        let shares = client.share(msg, &mut public_param.ctx);



        for server_idx in 0..num_shares{
            // Each server receives their share and commitment
            agg[server_idx].receive_share(&shares.shares[server_idx], &shares.randomness[server_idx], &shares.commitments[server_idx], &mut public_param.ctx);

            // Each server also recerives commitments for 
            agg[server_idx].receive_commitments(client_idx, &shares.commitments);
        }          
    }

    // Now a random server has received a message from another server
    // in v and r
    // It will complain the minute 
    for server_idx in 0..num_shares{
        let v = &BigNum::new().unwrap() + &agg[server_idx].agg_shares;
        let r =  &agg[server_idx].agg_randomness;
        let check = agg[0].receive_tally_broadcast(server_idx, &v, r, &mut public_param.ctx);
        // println!("Check good for {} is {}", server_idx, check);
        if check{
            agg[0].aggregate(v);            
        }        
    }
    println!("HISTOGRAM: {}:{},", sum_of_inputs, agg[0].ans);

}