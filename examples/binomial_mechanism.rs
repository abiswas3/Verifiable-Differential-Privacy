extern crate dp_client as ss;
use openssl::bn::BigNum;
use ss::server::Server;
// use rand::Rng;
use ss::utils::{print_vec};

fn main(){

    // Parameters 
    let security_parameter = 4;
    let num_candidates = 3; // Singe dim bin mean estimation for now
    let num_shares = 3; // num_servers
    let num_clients = 100;

    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    println!("{}\n\n", public_param);
    
    
    const EPOCH: u32 = 10;
    for epoch in 0..EPOCH{
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
                    // All the servers have updated their shares                                                                   
                    for server_idx in 0..num_shares{     
                        if server_idx == gen_server_idx{
                            agg[gen_server_idx].adapt_shares_for_morra_gen_server(dim, &share_of_shares[dim].shares[gen_server_idx], &share_of_shares[dim].randomness[gen_server_idx]);

                        }                   
                        else{
                            agg[server_idx].adapt_shares_for_morra_rec_server(dim, &share_of_shares[dim].shares[server_idx], &share_of_shares[dim].randomness[server_idx]);
                        }                                                
                    }
                    for server_idx1 in 0..num_shares{
                        for server_idx2 in 0..num_shares{
                            let is_gen_server = server_idx2 == gen_server_idx;
                            agg[server_idx1].adapt_coms(dim, server_idx2, is_gen_server, &mut public_param.ctx);
                        }
                    }
                }
            }
            // DP END
        }

        // POST RECONSTRUCT: 
        for dim in 0..num_candidates as usize{
            for server_idx in 0..num_shares{
                let  zero = BigNum::new().unwrap();
                let mut v = BigNum::new().unwrap();
                let mut r = BigNum::new().unwrap();
                
                v.mod_add(&zero, &agg[server_idx].agg_shares[dim], &public_param.q, &mut public_param.ctx).unwrap();
                r.mod_add(&zero, &agg[server_idx].agg_randomness[dim], &public_param.q, &mut public_param.ctx).unwrap();

                // println!("SERVER IDX : {} SHARE :{}, {}", server_idx, v, r);
                agg[0].receive_tally_broadcast(dim, server_idx, &v, &r, &mut public_param.ctx);
                agg[0].aggregate(dim, v, &mut public_param.ctx);
            }
            println!("");

        }    
        println!("POST DP NOISE RECONSTRUCTION: {}", epoch);
        print_vec(&agg[0].ans);        
        // EPOCH end
    }
    

    
}