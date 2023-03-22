use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

pub struct  ProofScalar{
    pub com: RistrettoPoint,
    pub e0 : Scalar, 
    pub e1 : Scalar, 
    pub e : Scalar, 
    pub v0: Scalar, 
    pub v1: Scalar, 
    pub d0: RistrettoPoint, 
    pub d1: RistrettoPoint,
}

// // Proof system for a vector of OR's and AND's
// pub struct  Proof{
//     pub coms: Vec<BigNum>,
//     pub e0 : BigNum, 
//     pub e1 : BigNum, 
//     pub e : BigNum, 
//     pub v0: BigNum, 
//     pub v1: BigNum, 
//     pub d0: BigNum, 
//     pub d1: BigNum,
// }

// impl Proof{

//     pub fn new(_coms: &Vec<BigNum>, _e0: &BigNum, _e1: &BigNum, _e: &BigNum, _v0: &BigNum, _v1: &BigNum, _d0: &BigNum, _d1: &BigNum)->Proof{

//         let e0 = &BigNum::new().unwrap() + _e0;
//         let e1 = &BigNum::new().unwrap() + _e1;
//         let e = &BigNum::new().unwrap() + _e;
//         let v0 = &BigNum::new().unwrap() + _v0;
//         let v1 = &BigNum::new().unwrap() + _v1;
//         let d0 = &BigNum::new().unwrap() + _d0;
//         let d1 = &BigNum::new().unwrap() + _d1;

//         let mut coms: Vec<BigNum> = Vec::new();
//         for c in _coms.iter(){
//             coms.push(c + &BigNum::new().unwrap());
//         }

//         Self{coms, e0, e1, e, v0, v1, d0, d1}
//     }
// }
