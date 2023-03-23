use criterion::{criterion_group, criterion_main, Criterion};
use dp_client as ss;
use curve25519_dalek::constants;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand_core::OsRng;
use openssl::bn::BigNum;
use::dp_client::generic_commitments::Commitment;
// use std::ops::Rem;

pub fn ecc_commitment(c: &mut Criterion) {

    let mut csprng = OsRng;
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let com = ss::generic_commitments::CurveCommitment{g, h};
    
    let msg: [u8;32] = ss::converters::u32_to_bytes(3);
    let r: Scalar = Scalar::random(&mut csprng);
    c.bench_function("curve coms", |b| b.iter(|| com.commit(&msg, r)));
}

pub fn ff_commitment(c: &mut Criterion) {

    let security_parameter = 256;
    let num_shares = 4; // num_servers (but will only use one server)
    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();  
    let ff_com = ss::finite_field_coms::Commitment::new(&public_param.p, &public_param.q, &public_param.g, &public_param.h);
    c.bench_function("ff coms", |b| b.iter(|| ff_com.commit(&BigNum::from_u32(3).unwrap(), &mut public_param.ctx)));
}

pub fn fiat_shamir_sigma_proof_creation(c: &mut Criterion){

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let coms = ss::generic_commitments::CurveCommitment{g, h};
    let num_shares = 2;

    let client = ss::client::Client{num_shares, coms};

    c.bench_function("proof-creation", |b| b.iter(|| client.coms.create_proof_0()));
}



pub fn fiat_shamir_sigma_proof_verification(c: &mut Criterion){
    
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let coms = ss::generic_commitments::CurveCommitment{g, h};
    let num_shares = 2;
    let client = ss::client::Client{num_shares, coms};
    let transcript = client.coms.create_proof_1();

    let board = ss::audit::Board{g, h};
    
    c.bench_function("proof-verification", |b| b.iter(|| board.verify(&transcript)));
}

// pub fn aggregation(c: &mut Criterion){
    
// }

// pub fn aggregation_multiplication(c: &mut Criterion){
    
// }


criterion_group!(benches, fiat_shamir_sigma_proof_creation, fiat_shamir_sigma_proof_verification);
criterion_main!(benches);