extern crate dp_client as ss;
use std::time::{Instant};


fn main(){

    // Script to benchamrk main bottle necks of counting query -- operations aren't exact. 

    // Parameters 
    let security_parameter = 256;
    // let num_candidates = 4; // Singe dim bin mean estimation for now
    let num_shares = 4; // num_servers (but will only use one server)
    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();
    // println!("{}", public_param);
    
    
    
    
    let base: i32 = 2; // an explicit type is required        
    let num_clients = base.pow(20); // M   
    let num_candidates = 2;
    let client = ss::verifiable_client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);
        
    let delta: f64 = 1.0/(base.pow(10) as f64);
    for i in 0..18{

        let n_b = base.pow(i);
        let float_nb = n_b as f64;
        let tmp = (2.0/(delta* float_nb) as f64).sqrt()*10.0;
        println!("Epsilon: {}", tmp);
        let now = Instant::now();                        

        let num_parallel_cores = 8;
        for _ in 0..n_b/num_parallel_cores{
            let _= client.create_proof_1(&mut public_param.ctx); // Verifier never figures this out    
        }
        let end = now.elapsed().as_micros();
        println!("Com proving time :{} mu(s) for {} coins", end, n_b);
        let end = now.elapsed().as_millis();
        println!("Com proving time :{} ms for {} coins", end, n_b);        

        let proofs = client.create_proof_1(&mut public_param.ctx); // Verifier never figures this out    
        let now = Instant::now();                        
        for _ in 0..n_b/num_parallel_cores{
            let board = ss::audit::Board::new(&proofs.com, 
                &public_param.p,
                &public_param.q,
                &public_param.g,
                &public_param.h,
                &proofs.e0,
                &proofs.e1,
                &proofs.e,
                &proofs.v0,
                &proofs.v1,
                &proofs.d0,
                &proofs.d1,
                num_shares);
                board.verify(&mut public_param.ctx);
        }
        let end = now.elapsed().as_micros();
        println!("Com verifying time :{} mu(s) for {} coins", end, n_b);
        let end = now.elapsed().as_millis();
        println!("Com verifying time :{} ms for {} coins\n", end, n_b);        

    }
    println!();
    // Morra sample n_b random values
    // sample them again and add them up
    let now = Instant::now();                        
    let mut aggregate = 0;
    for _ in 0..num_clients{
        aggregate = aggregate + 1;
    }
    let end = now.elapsed().as_micros();
    println!("Agg :{} mu(s) for {} clients", end, num_clients);
    let end = now.elapsed().as_millis();
    println!("Agg :{} ms for {} clients", end, num_clients);


    let now = Instant::now();                        
    let x = ss::utils::gen_random(&public_param.q).unwrap();
    let (c_x, r_x) = client.commit(&x, &mut public_param.ctx).unwrap();
    _ = client.open(&c_x, &x, &r_x, &mut public_param.ctx);
    println!("Single Com :{} mu(s)", end);
    let end = now.elapsed().as_millis();
    println!("Single Com :{} ms", end);
            
        
    
    
}