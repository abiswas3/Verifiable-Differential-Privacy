use criterion::{criterion_group, criterion_main, Criterion};
use dp_client as ss;
use curve25519_dalek::constants;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;
use::dp_client::generic_commitments::{Commitment, CurveCommitment};
// use std::ops::Rem;
use rand::Rng;

pub fn aux_ecc_commitments(com: &CurveCommitment){
    
    let msg: [u8;32] = rand::thread_rng().gen::<[u8; 32]>();
    println!("{:?}", msg);
    let mut csprng = OsRng;
    let r: Scalar = Scalar::random(&mut csprng);
    com.commit(&msg, r);
}
pub fn ecc_commitment(c: &mut Criterion) {

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let com = ss::generic_commitments::CurveCommitment{g, h};
    

    c.bench_function("Ristretto Coms", |b| b.iter(|| aux_ecc_commitments(&com)));
}

pub fn fiat_shamir_sigma_proof_creation(c: &mut Criterion){

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    
    let num_shares = 2;
    let client = ss::participants::Client::new(num_shares, g, h);
    let r = client.com.sample_randomness();
    c.bench_function("proof-creation", |b| b.iter(|| client.com.create_proof_0(r)));
}


pub fn fiat_shamir_sigma_proof_verification(c: &mut Criterion){
    
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
   
    let num_shares = 2;
    let client = ss::participants::Client::new(num_shares, g, h);
    let transcript = client.com.create_proof_1();

    let board = ss::participants::Board{g, h};    
    c.bench_function("proof-verification", |b| b.iter(|| board.verify(&transcript)));
}

pub fn aggregation(c: &mut Criterion){

    let num_clients = 100000;
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
   
    let num_shares = 2;
    let client = ss::participants::Client::new(num_shares, g, h);
    let mut inputs: Vec<(Scalar, Scalar)>= Vec::new();
    for _ in 0..num_clients{
        let (x,r) = client.send_input_to_sever();
        inputs.push((x, r));
    }

    c.bench_function("aggregation-n=10^6", |b| b.iter(|| inputs.iter()
    .fold((Scalar::zero(), Scalar::zero()), |(x_sum, r_sum), (x,r)| (x_sum+x, r_sum+r))));

}

pub fn aux_com_agg()->bool{


    let num_clients = 1000;
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;   
    let num_shares = 2;
    let client = ss::participants::Client::new(num_shares, g, h);
    let mut inputs: Vec<(Scalar, Scalar)>= Vec::new();
    let mut coms_to_inputs : Vec<RistrettoPoint> = Vec::new();    
    for _ in 0..num_clients{
        let (x,r) = client.send_input_to_sever();
        inputs.push((x, r));
        let com = client.com.commit(&x.to_bytes(), r);
        coms_to_inputs.push(com);
    }

    let (x_sum, r_sum) = inputs.iter()
    .fold((Scalar::zero(), Scalar::zero()), |(x_sum, r_sum), (x,r)| (x_sum+x, r_sum+r));
    
    let mut coms_sum = coms_to_inputs[0];
    for i in 1..coms_to_inputs.len(){
        coms_sum = coms_sum + coms_to_inputs[i];
    }

    let lhs = client.com.commit(&x_sum.as_bytes(), r_sum);
    let rhs = coms_sum;
    return lhs == rhs; 

}

pub fn com_agg(c: &mut Criterion){

    c.bench_function("com-aggregation-n=1000", |b| b.iter(|| aux_com_agg()));

}
criterion_group!(benches, ecc_commitment, fiat_shamir_sigma_proof_creation, fiat_shamir_sigma_proof_verification, aggregation, com_agg);
criterion_main!(benches);