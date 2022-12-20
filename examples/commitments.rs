extern crate dp_client as ss;

fn main(){

    use curve25519_dalek::constants;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;
    use rand_core::OsRng;

    use ss::converters::u32_to_bytes as helper;
    // use num_bigint::{BigUint};

    let h = RistrettoPoint::from_uniform_bytes(b"this is another secret that should never be disclosed to anyone ");
    let g = constants::RISTRETTO_BASEPOINT_POINT;
    let com = ss::elliptic::Commitment::new(g, h);

    let mut csprng = OsRng;
    let r1: Scalar = Scalar::random(&mut csprng);
    let r2: Scalar = Scalar::random(&mut csprng);
    let msg1: [u8;32] = helper(3);
    let msg2: [u8;32] = helper(5);


    let combined: [u8; 32] = ss::converters::add_byte_arrays(&msg1, &msg2).to_bytes();
    
    // let tmp = Scalar::from_canonical_bytes(combined).unwrap();
    println!("{:?}", ss::converters::as_u32_le(&combined));

    let r = r1 + r2;
    

    let c1 = com.commit(&msg1, r1);
    let c2 = com.commit(&msg2, r2);

    let c  = c1 + c2;

    println!("{:?}", com.open(c1, &msg1, r1));
    println!("{:?}", com.open(c2, &msg2, r2));
    println!("{:?}", com.open(c, &helper(8), r)); // Check homo-morphism
}