use curve25519_dalek::scalar::Scalar;
use openssl::bn::{BigNum};

pub fn bignum_to_bytes(x: BigNum)->[u8;32]{

    let mut tmp = [0 as u8; 32];
    for (i, byte) in x.to_vec().iter().enumerate(){
        tmp[i] = byte + 0;
    }   
    return tmp;
}

pub fn u32_to_bytes(x: u32)->[u8;32]{

    let mut ans: [u8; 32] = [0; 32];
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    ans[0] = b4;
    ans[1] = b3;
    ans[2] = b2;
    ans[3] = b1;
    return ans;
}

// pub fn as_u32_be(array: &[u8; 32]) -> u32 {

//     ((array[0] as u32) << 24) +
//     ((array[1] as u32) << 16) +
//     ((array[2] as u32) <<  8) +
//     ((array[3] as u32) <<  0)
// }

pub fn as_u32_le(array: &[u8; 32]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}


pub fn add_byte_arrays(a: &[u8; 32], b: &[u8; 32])->Scalar{

    let _a = a.clone();
    let _b = b.clone();
    let ans =  Scalar::from_canonical_bytes(_a).unwrap() + Scalar::from_canonical_bytes(_b).unwrap();
    return  ans;
    // return u32_to_bytes(as_u32_le(a) + as_u32_le(b))

}
