extern crate dp_client as ss;
use openssl::bn::BigNum;
use ss::simple_server::SimpleServer;
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
    let num_candidates = 2; // Singe dim bin mean estimation for now
    let num_shares = 4; // num_servers
    let num_clients = 1000;

    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    // println!("{}\n\n", public_param);
    

    let epochs = 10;
    for _ in 0..epochs{
        // CREATING SERVERS
        let mut agg = Vec::new();
        for _ in 0..num_shares{
            agg.push(SimpleServer::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h));
        }
        
        // Initialise a vector of histograms
        let mut truth = Vec::new();
        for _ in 0..num_candidates as usize{
            truth.push(0);
        }
        
        // let mut sum_of_inputs = 0;
        let mut now = Instant::now();
        for _ in 0..num_clients{
            let client = ss::simple_client::SimpleClient::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
            let msg = generate_random_vote(num_candidates as u32);
            truth[msg as usize] +=1;        

            // M x K : number of dims x number of servers
            let share_of_shares = client.vote(msg, &mut public_param.ctx); // there are M commitments for K servers
            
            // This is only after we have verified the shares are well formed.
            // Client has been accepted
            for (dim, shares) in share_of_shares.iter().enumerate(){        
                for server_idx in 0..num_shares{
                    // It needs to verify the code here.
                    agg[server_idx].receive_share(dim, &shares.shares[server_idx]);                                
                }                              
            }

            // At this point the client has distributed their shares to all servers
            // The servers should have all the information to verify if the client was well behaved
            // In reality every server runs the following code but for the sake of simplicity we assume 
            // server 0 is the honest server
            let mut broadcasted_z : Vec<BigNum> = Vec::new();
            let mut broadcasted_z_star : Vec<BigNum> = Vec::new();
            
            let r_vec = agg[0].generate_fresh_randomness();
            let _ = agg[0].generate_fresh_morra();
            for server_idx in 0..num_shares{
                
                // For the current client each server broadcasts messages for checking
                // Program crashes if servers have cheated
                let (z, z_star) = agg[server_idx].broadcast(&r_vec, &mut public_param.ctx);        
                        
                // Once the check was clean that server is able to 
                broadcasted_z.push(z);
                broadcasted_z_star.push(z_star);
            }
            // If this test fails: servers will adjust their shares accordingly
            _ = agg[0].sketching_test(&broadcasted_z, &broadcasted_z_star, &mut public_param.ctx);
        }
        let mut tmp = now.elapsed().as_millis();
        println!("Time to accept clients: {}", tmp);        

        now = Instant::now();
        for dim in 0..num_candidates as usize{
            for server_idx in 0..num_shares{
                // Each server_idx also needs to add binomial noise to v so might as well set it to 0 or something
                let  zero = BigNum::new().unwrap();
                let mut v = BigNum::new().unwrap();                        
                v.mod_add(&zero, &agg[server_idx].agg_shares[dim], &public_param.q, &mut public_param.ctx).unwrap();
                agg[0].aggregate(dim, v);
            }
        }    
        tmp = now.elapsed().as_micros();
        println!("Aggregate time: {} micro seconds \n", tmp); 
    }
         

}