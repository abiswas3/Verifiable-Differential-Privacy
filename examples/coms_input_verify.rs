extern crate dp_client as ss;
use openssl::bn::BigNum;
use std::time::{Instant};
use ss::public_parameters::PublicParams;
use ss::utils::{gen_random};
// A full simulation without the DP noise
fn main() {
        // Parameters 
        let security_parameter = 256; // number bits of security to use
        let num_shares = 4; // K    
        let mut public_param = PublicParams::new(security_parameter, num_shares).unwrap();
    
        let mut time: u128 = 0;
        let num_trials: usize = 1000;
        for _ in 0..num_trials{
            let now = Instant::now();
            let mut tmp = BigNum::new().unwrap();
            let r = gen_random(&public_param.q).unwrap();
            _ = tmp.mod_exp(&public_param.g, &r, &public_param.p, &mut public_param.ctx);
            let end = now.elapsed().as_micros();            
            time += end;
        }
        println!("Elapsed time over {} trials: {},", num_trials, time/(num_trials as u128));
}
