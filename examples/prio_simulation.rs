extern crate dp_client as ss;
use openssl::bn::BigNum;
use std::ops::Rem;
use ss::public_parameters::PublicParams;
use ss::prio::Server;
use ss::verifiable_client::Client;
use ss::beaver_triple::BeaverTriple;
use std::time::{Instant};


fn main(){


    let security_parameter = 256;
    let num_shares = 3; // num_servers
    let mut public_param = PublicParams::new(security_parameter, num_shares).unwrap();
    // println!("{}", public_param);

    for i in 1..14{
        let base: i32 = 2; // an explicit type is required        
        let num_candidates = base.pow(i) as usize;
        let mut elapsed_time = 0;
        let client = Client::new(num_shares, num_candidates as u32,  &public_param.q);    
        let choice = client.generate_fake_vote();       
        let vote = client.vote(choice, &mut public_param.ctx);
    
        let mut servers = Vec::new();
        for _ in 0..num_shares{
            let server = Server::new(num_shares, num_candidates,&public_param.q);
            servers.push(server);
        }
        // Pre-processing
        let beaver_triples = BeaverTriple::new(num_shares, &public_param.q, &mut public_param.ctx);
        let randomness = servers[0].generate_fresh_randomness();
        // end
        
        // ROUND 1 of MPC
        let mut d = BigNum::new().unwrap();
        let mut e = BigNum::new().unwrap();    
        for server_idx in 0..num_shares{
            let mut shares = Vec::new(); // Get the shares for server i (copy it over)
            for coord in 0..num_candidates{
                shares.push(&vote[coord].shares[server_idx] + &BigNum::new().unwrap());
            }        
            let now = Instant::now();
            let (d_i, e_i) = servers[server_idx].create_sketch_share_one(&shares, &randomness, &beaver_triples.a_shares[server_idx], &beaver_triples.b_shares[server_idx], &mut public_param.ctx);
            d = (&d + &d_i).rem(&public_param.q);
            e = (&e + &e_i).rem(&public_param.q);
            let end = now.elapsed().as_micros();
            elapsed_time += end;        
        }

        // ROUND 2 of MPC
        let mut ans = BigNum::new().unwrap();
        for server_idx in 0..num_shares{
            let mut shares = Vec::new(); // Get the shares for server i (copy it over)
            for coord in 0..num_candidates{
                shares.push(&vote[coord].shares[server_idx] + &BigNum::new().unwrap());
            }
            let now = Instant::now();
            let tmp = servers[server_idx].create_sketch_share_two(&shares, &randomness, &beaver_triples.a_shares[server_idx], &beaver_triples.b_shares[server_idx], &beaver_triples.c_shares[server_idx], &e, &d, &mut public_param.ctx);
            ans = (&ans + &tmp).rem(&public_param.q);
            let end = now.elapsed().as_micros();
            elapsed_time += end;        
        }    
        elapsed_time = elapsed_time/(num_shares as u128); // They all run in parallel

        let now = Instant::now();
        ans = (&ans +  &(&d*&e)).rem(&public_param.q);
        let end = now.elapsed().as_micros();
        elapsed_time += end;
        println!("{}:{},", num_candidates, elapsed_time);
        assert_eq!(ans, BigNum::new().unwrap());        
    }
}