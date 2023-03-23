extern crate dp_client as ss;
use curve25519_dalek::{ristretto::RistrettoPoint};
use curve25519_dalek::constants;
use ss::generic_commitments::Commitment;
use ss::sigma_ff::ProofScalar;
use std::time::{Instant};

fn main(){

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let num_shares = 2;

    // Clients send input to servers while publicly committing to input
    let client = ss::participants::Client::new(num_shares, g, h);

    // epsilon = 10*np.sqrt(1/n*np.log(2/delta))
    // delta = 10^-10
    let private_coins = [256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144];
    let corresponding_epsilons = [3.0438846293698814, 2.1523514625769438, 1.5219423146849407, 1.0761757312884719, 0.7609711573424703, 0.5380878656442359, 0.38048557867123517, 0.26904393282211797, 0.19024278933561758, 0.13452196641105899, 0.09512139466780879];

    for i in 0..private_coins.len(){
        let n_b = private_coins[i];
        let now = Instant::now();
        for _ in 0..n_b{
            let r = client.com.sample_randomness();
            let _ = client.com.create_proof_0(r);
        }
        let end = now.elapsed().as_millis();
        println!("Time taken to sequentially create proofs for epsilon: {} or ({} coins) := {} ms", corresponding_epsilons[i], n_b, end);
    }

    println!();
    let verifier = ss::participants::Board{g, h};
    for i in 0..private_coins.len(){
        let n_b = private_coins[i];
        let mut proofs: Vec<ProofScalar> = Vec::new();
        for _ in 0..n_b{        
            let r = client.com.sample_randomness();
            proofs.push(client.com.create_proof_0(r));
        }    
    
        let now = Instant::now();
        for _ in 0..n_b{                
            _ = verifier.verify(&proofs[i]);
        }   
        let end = now.elapsed().as_millis();
        println!("Time taken to sequentially verify proofs for epsilon: {} or ({} coins) := {} ms", corresponding_epsilons[i], n_b, end);
    }


}