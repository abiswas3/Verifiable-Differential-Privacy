extern crate dp_client as ss;
use openssl::bn::BigNum;
use std::time::{Instant};
use ss::public_parameters::PublicParams;
use ss::client::Client;

// A full simulation without the DP noise
fn main() {
        // Parameters 
        let security_parameter = 256; // number bits of security to use (internally multiplied by 2)
        let num_candidates = 5; // M
        let num_shares = 4; // K
    
        let mut public_param = PublicParams::new(security_parameter, num_shares).unwrap();
        let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
    
        let now = Instant::now();
        let _ = client.commit(&BigNum::new().unwrap(), &mut public_param.ctx).unwrap();
        let end = now.elapsed().as_micros();
        println!("Elapsed time: {},", end);
}
