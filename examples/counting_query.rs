extern crate dp_client as ss;
use std::time::{Instant};
use std::ops::Rem;
use openssl::bn::BigNum;


fn main(){

    // Script to understand main bottle necks of counting query in performance-- operations aren't exact. 

    // Parameters START
    let security_parameter = 3072;
    let num_shares = 4; // num_servers (but will only use one server)
    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();    
    let base: i32 = 2; // an explicit type is required        
    let num_clients = base.pow(20);    
    let num_candidates = 2; // M
    let num_parallel_cores = 8;

    // Parameters START

    let client = ss::verifiable_client::Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);        
    let delta: f64 = 1.0/(base.pow(10) as f64);
    println!("Epsilon:\tProof Creation\tProof Verification\tNumber Of Coins\tMorra");
    for i in 0..22{

        let n_b = base.pow(i);
        let float_nb = n_b as f64;
        let tmp = (2.0/(delta* float_nb) as f64).sqrt()*10.0; // Epsilon
        print!("{}:\t", tmp);
        let now = Instant::now();                        

        for _ in 0..n_b/num_parallel_cores{
            let _= client.create_proof_1(&mut public_param.ctx); // Verifier never figures this out    
        }
        let end = now.elapsed().as_millis();
        print!("[{}",end);


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
        let end = now.elapsed().as_millis();
        print!(",\t{},\t{}", end, n_b);

        // Morra sample n_b random values
        // sample them again and add them up
        let now = Instant::now();   
        for _ in 0..n_b/num_parallel_cores{
            let x = ss::utils::gen_random(&public_param.q).unwrap();
            let (c_x, r_x) = client.commit(&x, &mut public_param.ctx).unwrap();    
            _ = client.open(&c_x, &x, &r_x, &mut public_param.ctx);
            let mut aggregate = BigNum::new().unwrap();

            // Aggregate final answer
            for _ in 0..num_shares{
                let x = ss::utils::gen_random(&public_param.q).unwrap();
                aggregate = (&aggregate + &x).rem(&public_param.q);
            }
        }
        let end = now.elapsed().as_millis();
        print!(",\t{}]\n", end);        
    }    


    // Compute how long it takes aggregate n BigNum's
    let now = Instant::now();                        
    let mut aggregate = BigNum::new().unwrap();
    for _ in 0..num_clients{
        let x = ss::utils::gen_random(&public_param.q).unwrap();
        aggregate = (&aggregate + &x).rem(&public_param.q);
    }
    let end = now.elapsed().as_millis();
    println!("Agg :{} ms for {} clients", (end as f64/num_parallel_cores as f64), num_clients);


    // Verification involves computing 1 commitment and n + n_b mulitplies
    // Time taken to commmit and open
    let now = Instant::now();                        
    let x = ss::utils::gen_random(&public_param.q).unwrap();
    let (c_x, r_x) = client.commit(&x, &mut public_param.ctx).unwrap();
    _ = client.open(&c_x, &x, &r_x, &mut public_param.ctx);

    // Time taken to perform group multiply operations
    let mut aggregate = BigNum::from_u32(1).unwrap();
    for _ in 0..num_clients{
        let x = ss::utils::gen_random(&public_param.q).unwrap();
        aggregate = (&aggregate * &x).rem(&public_param.p);
    }
    let end = now.elapsed().as_millis();
    println!("Single Com and group multiply:{} ms", (end as f64/num_parallel_cores as f64));
    
}

