extern crate dp_client as ss;

fn main(){

    use curve25519_dalek::constants;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;
    use rand_core::OsRng;
    use ss::converters::{u32_to_bytes};



    

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let com = ss::elliptic::Commitment::new(g, h);

    let num_trials = 10000;
    let (time_exp, time_mult) = com.get_timing_stats(num_trials);
    println!("Average over {} is {}, {}", num_trials, time_exp, time_mult);

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
}