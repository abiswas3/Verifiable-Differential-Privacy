extern crate dp_client as ss;
use openssl::bn::BigNum;
use std::ops::Rem;
use ss::public_parameters::PublicParams;
use ss::poplar::Server;
use ss::verifiable_client::Client;
use ss::beaver_triple::BeaverTriple;
use std::time::{Instant};


fn main(){

    

    let security_parameter = 256;
    let num_shares = 3; // num_servers
    let mut public_param = PublicParams::new(security_parameter, num_shares).unwrap();
    // println!("{}", public_param);

    for i in 0..14{
        let base: i32 = 2; // an explicit type is required        
        let num_candidates = base.pow(i) as usize;
        let mut elapsed_time = 0;

        let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);    
        let choice = client.generate_random_vote(num_candidates as u32);       
        let vote = client.vote(choice, &mut public_param.ctx);
        let kvote = client.poplar_vote(choice, &mut public_param.ctx);
    
        let mut servers = Vec::new();
        for _ in 0..num_shares{
            let server = Server::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
            servers.push(server);
        }
    
        // Pre-processing: Client has a,b and c
        let beaver_triples = BeaverTriple::new(num_shares, &public_param.q, &mut public_param.ctx);
        let randomness = servers[0].generate_fresh_randomness();
        
        // Servers never see a, b, c in plain
        let a = beaver_triples.a_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let b = beaver_triples.b_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let c = beaver_triples.c_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        
        let (ucase_a_share, ucase_b_share) = client.get_ab_for_poplar(&a, &b, &c, &mut public_param.ctx);

        // ROUND 1 of MPC
        let mut ucase_z = BigNum::new().unwrap();
        let mut ucase_z_star = BigNum::new().unwrap();
        let mut ucase_z_star_star = BigNum::new().unwrap();
        for server_idx in 0..num_shares{
            let mut shares = Vec::new(); // Get the shares for server i (copy it over)
            let mut k_shares =  Vec::new(); 
            for coord in 0..num_candidates{
                shares.push(&vote[coord].shares[server_idx] + &BigNum::new().unwrap());
                k_shares.push(&kvote[coord].shares[server_idx] + &BigNum::new().unwrap());
            }
            let now = Instant::now();
            let (z, z_star, z_star_star) = servers[server_idx].round_one_verification(&shares, &k_shares, &randomness, &beaver_triples.a_shares[server_idx], &beaver_triples.b_shares[server_idx], &beaver_triples.c_shares[server_idx], &mut public_param.ctx);
            ucase_z = (&ucase_z + &z).rem(&public_param.q);
            ucase_z_star = (&ucase_z_star + &z_star).rem(&public_param.q);
            ucase_z_star_star = (&ucase_z_star_star + &z_star_star).rem(&public_param.q);
            let end = now.elapsed().as_micros();
            elapsed_time += end;        

        }
    
        // ROUND 2 of MPC
        let mut ans = BigNum::new().unwrap();
        for server_idx in 0..num_shares{
            let now = Instant::now();
            let shr_output = servers[server_idx].round_two_verification(&ucase_a_share.shares[server_idx], &ucase_b_share.shares[server_idx], &ucase_z);
            ans = (&ans + &shr_output).rem(&public_param.q);
            let end = now.elapsed().as_micros();
            elapsed_time += end;        
        }
        
        elapsed_time = elapsed_time/(num_shares as u128); // They all run in parallel

        let now = Instant::now();
        let pos = (&ans + &(&ucase_z*&ucase_z)).rem(&public_param.q);
        let neg = &ucase_z_star + &ucase_z_star_star;
        let mut output = BigNum::new().unwrap();
        _ = output.mod_sub(&pos, &neg, &public_param.q, &mut public_param.ctx);
        let end = now.elapsed().as_micros();
        elapsed_time += end;        
        println!("{}:{},", num_candidates, elapsed_time);

        assert_eq!(output, BigNum::new().unwrap());
    
    }
}