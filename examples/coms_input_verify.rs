extern crate dp_client as ss;
use openssl::bn::BigNum;
use ss::server::Server;
use rand::Rng;
use std::time::{Instant};



// A full simulation without the DP noise 
fn generate_random_vote(num_candidates: u32)->u32{

    let mut rng = rand::thread_rng();
    return rng.gen_range(0..num_candidates);
}
fn main(){

    // Parameters 
    let security_parameter = 256;
    
    // println!("{}\n\n", public_param);
    

    println!("Varying Shares");
    for num_shares in [2, 4, 8, 16]{
        let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
        let num_candidates = 16; // Singe dim bin mean estimation for now

        // CREATING SERVERS
        let mut agg = Vec::new();
        for _ in 0..num_shares{
            agg.push(Server::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h));
        }

        let client = ss::client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let msg = generate_random_vote(num_candidates as u32);     

        // M x K : number of dims x number of servers
        let share_of_shares = client.vote(msg, &mut public_param.ctx); // there are M commitments for K servers
        
        // This is only after we have verified the shares are well formed.
        // Client has been accepted
        for (dim, shares) in share_of_shares.iter().enumerate(){        
            for server_idx in 0..num_shares{
                // It needs to verify the code here.
                agg[server_idx].receive_share(dim, &shares.shares[server_idx], &shares.randomness[server_idx], &shares.commitments[server_idx], &mut public_param.ctx);                
                
                // Each server also receives commitments for all other shares for all other servers
                // For that particular dimension
                agg[server_idx].receive_commitments(dim,  &shares.commitments);                
            }                              
        }

        // At this point the client has distributed their shares to all servers
        // The servers should have all the information to verify if the client was well behaved
        // In reality every server runs the following code but for the sake of simplicity we assume 
        // server 0 is the honest server
        let mut broadcasted_z : Vec<BigNum> = Vec::new();
        let mut broadcasted_z_star : Vec<BigNum> = Vec::new();
        let mut broadcasted_t : Vec<BigNum> = Vec::new();
        let now = Instant::now();

        let r_vec = agg[0].generate_fresh_randomness();
        let _ = agg[0].generate_fresh_morra();
        for server_idx in 0..num_shares{
            
            // For the current client each server broadcasts messages for checking
            // Program crashes if servers have cheated
            let (z, z_star, t, t_star) = agg[server_idx].broadcast(&r_vec, &mut public_param.ctx);        
            agg[0].verify_sketching_messages(server_idx, &r_vec, &z, &z_star, &t, &t_star, &mut public_param.ctx);
            
            // Once the check was clean that server is able to 
            broadcasted_z.push(z);
            broadcasted_z_star.push(z_star);
            broadcasted_t.push(t);
        }
        // If this test fails: servers will adjust their shares accordingly
        _ = agg[0].sketching_test(&broadcasted_z, &broadcasted_z_star, &mut public_param.ctx);        
        let tmp = now.elapsed().as_millis();        
        print!("{}:{}, ", num_shares, tmp);

    }


    println!("\nVarying Dimensions");
    for num_candidates in [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]{
        
        let num_shares = 4; // Singe dim bin mean estimation for now
        let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
        
        // CREATING SERVERS
        let mut agg = Vec::new();
        for _ in 0..num_shares{
            agg.push(Server::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h));
        }

        let client = ss::client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let msg = generate_random_vote(num_candidates as u32);     

        // M x K : number of dims x number of servers
        let share_of_shares = client.vote(msg, &mut public_param.ctx); // there are M commitments for K servers
        
        // This is only after we have verified the shares are well formed.
        // Client has been accepted
        for (dim, shares) in share_of_shares.iter().enumerate(){        
            for server_idx in 0..num_shares{
                // It needs to verify the code here.
                agg[server_idx].receive_share(dim, &shares.shares[server_idx], &shares.randomness[server_idx], &shares.commitments[server_idx], &mut public_param.ctx);                
                
                // Each server also receives commitments for all other shares for all other servers
                // For that particular dimension
                agg[server_idx].receive_commitments(dim,  &shares.commitments);                
            }                              
        }

        // At this point the client has distributed their shares to all servers
        // The servers should have all the information to verify if the client was well behaved
        // In reality every server runs the following code but for the sake of simplicity we assume 
        // server 0 is the honest server
        let mut broadcasted_z : Vec<BigNum> = Vec::new();
        let mut broadcasted_z_star : Vec<BigNum> = Vec::new();
        let mut broadcasted_t : Vec<BigNum> = Vec::new();
        let now = Instant::now();

        let r_vec = agg[0].generate_fresh_randomness();
        let _ = agg[0].generate_fresh_morra();
        for server_idx in 0..num_shares{
            
            // For the current client each server broadcasts messages for checking
            // Program crashes if servers have cheated
            let (z, z_star, t, t_star) = agg[server_idx].broadcast(&r_vec, &mut public_param.ctx);        
            agg[0].verify_sketching_messages(server_idx, &r_vec, &z, &z_star, &t, &t_star, &mut public_param.ctx);
            
            // Once the check was clean that server is able to 
            broadcasted_z.push(z);
            broadcasted_z_star.push(z_star);
            broadcasted_t.push(t);
        }
        // If this test fails: servers will adjust their shares accordingly
        _ = agg[0].sketching_test(&broadcasted_z, &broadcasted_z_star, &mut public_param.ctx);        
        let tmp = now.elapsed().as_millis();    
        print!("{}:{}, ", num_candidates, tmp);
    
    }    
    println!();

}