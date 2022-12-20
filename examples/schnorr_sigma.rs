extern crate dp_client as ss;
use openssl::bn::{BigNum};
use std::env;

fn main(){
    env::set_var("RUST_BACKTRACE", "1");

    // Parameters 
    let security_parameter = 8;
    let num_candidates = 2; // Singe dim bin mean estimation for now
    let num_shares = 2; // num_servers    
    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    println!("{}", public_param);

    let num_trials = 1000;
    for _ in 0..num_trials{

        let client = ss::verifiable_client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        let share_1 = client.share(1, &mut public_param.ctx);
        let share_0 = client.share(0, &mut public_param.ctx);

        let mut recons_share = BigNum::new().unwrap();
        let mut recons_rand = BigNum::new().unwrap();
        let mut recons_com = BigNum::from_u32(1).unwrap();

        for server_idx in 0..num_shares{
            recons_share = &(&recons_share + &share_1.shares[server_idx]) % &public_param.q;
            recons_rand = &(&recons_rand + &share_1.randomness[server_idx]) % &public_param.q;
            recons_com = &(&recons_com * &share_1.commitments[server_idx])% &public_param.p;
        }

        let check = client.open(&recons_com, &recons_share, &recons_rand, &mut public_param.ctx).unwrap();
        assert_eq!(true, check); // check commitment openning is good
        assert_eq!(recons_share, BigNum::from_u32(1).unwrap()); // check the secret is actually 1

        //----------------------------------PROOF STARTS----------------------------------------
        let (e0, e1, e, v0, v1, d0, d1) = client.create_cds94_proof_for_1(&share_1, &mut public_param.ctx);        
        let board = ss::audit::Board::new(&share_1.commitments, 
            &public_param.p,
            &public_param.q,
            &public_param.g,
            &public_param.h,
            &e0, 
            &e1, 
            &e, 
            &v0, 
            &v1, 
            &d0, 
            &d1, 
            num_shares);        
        
        board.verify(&mut public_param.ctx);

        let (e0, e1, e, v0, v1, d0, d1) = client.create_cds94_proof_for_0(&share_0, &mut public_param.ctx);        
        let board = ss::audit::Board::new(&share_0.commitments, 
            &public_param.p,
            &public_param.q,
            &public_param.g,
            &public_param.h,
            &e0, 
            &e1, 
            &e, 
            &v0, 
            &v1, 
            &d0, 
            &d1, 
            num_shares);        
        board.verify(&mut public_param.ctx);
    }
}