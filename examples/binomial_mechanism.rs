extern crate dp_client as ss;
use openssl::bn::BigNum;
use ss::server::Server;
// use rand::Rng;
use ss::utils::{print_vec};
use std::ops::Rem;



fn main(){

    // Parameters 
    let security_parameter = 4;
    let num_candidates = 3; // Singe dim bin mean estimation for now
    let num_shares = 4; // num_servers
    let num_clients = 100;

    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    println!("{}\n\n", public_param);
    
    
    const EPOCH: u32 = 1;
    for _ in 0..EPOCH{
        // CREATING SERVERS
        let mut agg = Vec::new();
        for _ in 0..num_shares{
            agg.push(Server::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h));
        }
        for _ in 0..num_clients{
            let  gen_server_idx = 1;
            let share_of_shares = agg[gen_server_idx].generate_shares_for_low_qual_bit(&mut public_param.ctx);
            let morra_bits = agg[gen_server_idx].generate_fresh_morra();

            // Receive shares like normal
            for (dim, shares) in share_of_shares.iter().enumerate(){        
                for server_idx in 0..num_shares{                    
                    agg[server_idx].receive_share(dim, &shares.shares[server_idx], &shares.randomness[server_idx], &shares.commitments[server_idx], &mut public_param.ctx);                                   
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
            let r_vec = agg[0].generate_fresh_randomness();
            let _ = agg[0].generate_fresh_morra();
            for server_idx in 0..num_shares{
            
                // For the current client each server broadcasts messages for checking
                // Program crashes if servers have cheated
                let (z, z_star, t, t_star) = agg[server_idx].broadcast(&r_vec, &mut public_param.ctx);  
                
                // Technically all the servers do this but we're just assuming 0 is the honest server
                agg[0].verify_sketching_messages(server_idx, &r_vec, &z, &z_star, &t, &t_star, &mut public_param.ctx);
                
                // Once the check was clean that server is able to 
                broadcasted_z.push(z);
                broadcasted_z_star.push(z_star);
                broadcasted_t.push(t);
            }

            // If this test fails: servers will adjust their shares accordingly by exlcuding the client being processed
            _ = agg[0].sketching_test(&broadcasted_z, &broadcasted_z_star, &mut public_param.ctx);                  

            // DP
            
            for dim in 0..num_candidates{
                
                if morra_bits[dim] == 1{                                                        
                    agg[gen_server_idx].adapt_shares_for_morra(dim, &share_of_shares[dim].shares[gen_server_idx], &share_of_shares[dim].randomness[gen_server_idx], &mut public_param.ctx);
                    for server_idx in 0..num_shares{                        
                        agg[server_idx].adapt_coms(dim, server_idx, &mut public_param.ctx);                        
                    }
                }
            }
            // DP END           
            break
        }

        // RECONSTRUCT: 
        for dim in 0..num_candidates as usize{
            for server_idx in 0..num_shares{
                let v = &BigNum::new().unwrap() + &agg[server_idx].agg_shares[dim];
                let r =  &agg[server_idx].agg_randomness[dim];            
                agg[0].receive_tally_broadcast(dim, server_idx, &v, r, &mut public_param.ctx);
                agg[0].aggregate(dim, v, &mut public_param.ctx);
            }            
        }    
        println!("RECONSTRUCTION");
        print_vec(&agg[0].ans);
        
    }
    

    
}