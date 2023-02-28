use openssl::bn::BigNum;
use std::ops::Rem;

extern crate dp_client as ss;

fn main(){

    use curve25519_dalek::constants;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;
    use rand_core::OsRng;
    use ss::converters::{u32_to_bytes};
    
    println!("ECC Commitments");
    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let com = ss::elliptic::Commitment::new(g, h);

    let num_trials = 10000;
    let (time_exp, time_mult) = com.get_timing_stats(num_trials);
    println!("Average over {} trials is {}, {}", num_trials, time_exp, time_mult);

    println!("Checking Homomorphism");
    let mut csprng = OsRng;
    let r1: Scalar = Scalar::random(&mut csprng);    
    let msg1: [u8;32] = u32_to_bytes(3);
    // let msg1 = bignum_to_bytes(BigNum::from_u32(3).unwrap());

    let msg2: [u8;32] = u32_to_bytes(5);
    let r2: Scalar = Scalar::random(&mut csprng);

    // let _a = Commitment::from_vec(BigNum::from_u32(1).unwrap().to_vec());
    // let combined: [u8; 32] = ss::converters::add_byte_arrays(&msg1, &msg2).to_bytes();    
    // let tmp = Scalar::from_canonical_bytes(combined).unwrap();
    // println!("{:?}", ss::converters::as_u32_le(&combined));
    
    let c1 = com.commit(&msg1, r1);
    let c2 = com.commit(&msg2, r2);

    let c  = c1 + c2;
    let r = r1 + r2;

    assert_eq!(true, com.open(c1, &msg1, r1));
    assert_eq!(true, com.open(c2, &msg2, r2));
    assert_eq!(true, com.open(c, &u32_to_bytes(8), r));

    println!("Finite Field Commitments");
    let security_parameter = 512;
    let num_shares = 4; // num_servers (but will only use one server)
    let mut public_param = ss::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap();  
    let ff_com = ss::finite_field_coms::Commitment::new(&public_param.p, &public_param.q, &public_param.g, &public_param.h);

    let (c1, r1) = ff_com.commit(&BigNum::from_u32(3).unwrap(), &mut public_param.ctx).unwrap();
    let (c2, r2) = ff_com.commit(&BigNum::from_u32(5).unwrap(), &mut public_param.ctx).unwrap();    
    
    let x = &BigNum::from_u32(3).unwrap() + &BigNum::from_u32(5).unwrap();
    let r = (&r1 + &r2).rem(&public_param.q);
    let c = ff_com.helper(&x, &r, &mut public_param.ctx).unwrap();
    assert_eq!(c, (&c1*&c2).rem(&public_param.p));


}