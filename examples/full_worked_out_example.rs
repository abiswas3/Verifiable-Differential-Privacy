use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use curve25519_dalek::constants;
use ss::generic_commitments::Commitment;
extern crate dp_client as ss;
use coinflip::flip;
// use std::time::{Instant};

fn main(){

    // Public Paramters
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let num_clients = 1000;
    let num_shares = 2;

    // Clients send input to servers while publicly committing to input
    let client = ss::participants::Client::new(num_shares, g, h);
    let mut inputs: Vec<(Scalar, Scalar)>= Vec::new();
    let mut coms_to_inputs : Vec<RistrettoPoint> = Vec::new();    
    for _ in 0..num_clients{
        let (x,r) = client.send_input_to_sever();
        inputs.push((x, r));
        let com = client.com.commit(x, r);
        coms_to_inputs.push(com);
    }

    let (x_sum, r_sum) = inputs.iter()
    .fold((Scalar::zero(), Scalar::zero()), |(x_sum, r_sum), (x,r)| (x_sum+x, r_sum+r));
    
    let mut coms_sum = coms_to_inputs[0];
    for i in 1..coms_to_inputs.len(){
        coms_sum = coms_sum + coms_to_inputs[i];
    }

    let lhs = client.com.commit(x_sum, r_sum);
    let rhs = coms_sum;
    
    assert_eq!(lhs, rhs);
    

    let server = ss::participants::Server::new(num_shares, g, h);
    let verifier = ss::participants::Board{g, h};

    let n_b = 100;

    let mut private_bits: Vec<(Scalar, Scalar)>= Vec::new();
    let mut coms_to_bits : Vec<RistrettoPoint> = Vec::new();    
    for _ in 0..n_b{

        // Prover commits to a bit
        let r = server.com.sample_randomness();
        let transcript = server.com.create_proof_0(r);
        // Verifier verifies if the bit is 0 or 1
        _ = verifier.verify(&transcript);

        // Morra
        let b = flip();
        if b{
            private_bits.push((Scalar::one(), Scalar::one() - r));
            let com_one = server.com.commit(Scalar::one(),
                                                          Scalar::one());
            coms_to_bits.push(&com_one - &transcript.com);
        }
        else{
            private_bits.push((Scalar::zero(), r));
            coms_to_bits.push(transcript.com);
        }        
    }

    let (v_sum, s_sum) = private_bits.iter()
    .fold((Scalar::zero(), Scalar::zero()), |(x_sum, r_sum), (x,r)| (x_sum+x, r_sum+r));
    
    let mut v_coms_sum = coms_to_bits[0];
    for i in 1..coms_to_bits.len(){
        v_coms_sum = v_coms_sum + coms_to_bits[i];
    }

    // THIS IS THE OUTPUT OF THE PROVER
    let x = x_sum + v_sum; // Input Agg + Noise
    let r = r_sum + s_sum; // Keys added up

    let lhs = client.com.commit(x, r);
    let rhs = coms_sum + v_coms_sum;

    assert_eq!(lhs, rhs);
    println!("All worked out really nicely");    

    
}