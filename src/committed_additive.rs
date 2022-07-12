use openssl::bn::{BigNum, BigNumContext};
use openssl::error::ErrorStack;
// use std::fmt;
// use rand::distributions::{Distribution, Uniform};

// TODO: Change the primes to BigNum support
pub struct CommitedAdditiveSecretSharing {
    pub num_shares: usize,
    pub p: BigNum,
    pub q: BigNum,
    pub g: BigNum,
    pub h: BigNum,
    ctx: BigNumContext,    
}

fn gen_random(limit: &BigNum) -> Result<BigNum, ErrorStack> {
    // generate random bignum between 1, limit-1
    let one = BigNum::from_u32(1)?;
    let mut r = BigNum::new()?;
    let mut tmp1 = BigNum::new()?;
    tmp1.checked_sub(limit, &one)?;
    let mut tmp2 = BigNum::new()?;
    tmp2.checked_add(&r, &one)?;
    tmp1.rand_range(&mut r)?;
    Ok(r)
}


fn calculate_q(p: &BigNum, ctx: &mut BigNumContext) -> Result<BigNum, ErrorStack> {
    // generate q = 2p + 1
    let mut q = BigNum::new()?;
    let one = BigNum::from_u32(1)?;
    let two = BigNum::from_u32(2)?;
    let mut tmp = BigNum::new()?;
    tmp.checked_mul(p, &two, ctx)?;
    q.checked_add(&tmp, &one)?;
    Ok(q)
}


impl CommitedAdditiveSecretSharing{

    pub fn new(security: i32, num_shares: usize) -> Result<CommitedAdditiveSecretSharing, ErrorStack> {
        // create context to manage the bignum
        let mut ctx = BigNumContext::new()?;
   
        // generate prime number with 2*security bits
        let mut p = BigNum::new()?;
        p.generate_prime(2 * security, false, None, None)?;

        // calculate q from p, where q = 2p + 1
        // this ensures that every element in Z_q is a generator
        let q = calculate_q(&p, &mut ctx)?;

        // generate random g
        let g = gen_random(&p)?;

        // generate random secret alpha
        let alpha = gen_random(&p)?;

        // calculate h = pow(g, alpha, p)
        let mut h = BigNum::new()?;

        h.mod_exp(&g, &alpha, &p, &mut ctx)?;

        Ok(Self { num_shares, p, q, g, h, ctx })
    }


    pub fn share(&mut self, _secret: u32)->Vec<BigNum>{

        let mut shares = Vec::new();
        for _ in 1..(self.num_shares){            
            shares.push(gen_random(&self.q).unwrap());
            
        }
        
        let secret = BigNum::from_u32(_secret).unwrap();
        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut last_share = BigNum::new().unwrap();
        _ = last_share.mod_sub(&secret, &total, &self.q, &mut self.ctx);
        
        shares.push(last_share);
        return shares;
    }

    pub fn reconstruct(&mut self, shares: Vec<BigNum>)->BigNum{

        let total = shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
        let mut reconstructed_value = BigNum::new().unwrap();
        _ = reconstructed_value.nnmod(&total, &self.q, &mut self.ctx);
        return reconstructed_value;
    }


    // fn helper(&mut self, x: u32, r: &BigNum) -> Result<BigNum, ErrorStack> {
    //     // returns g^x1h^r
    //     let x1 = BigNum::from_u32(x)?;
    //     let mut c = BigNum::new()?;
    //     let mut tmp3 = BigNum::new()?;
    //     let mut tmp4 = BigNum::new()?;
    //     tmp3.mod_exp(&self.g, &x1, &self.q, &mut self.ctx)?;
    //     tmp4.mod_exp(&self.h, r, &self.q, &mut self.ctx)?;
    //     c.mod_mul(&tmp3, &tmp4, &self.q, &mut self.ctx)?;
    //     return Ok(c);
    // }    
}

// impl AdditiveSecretSharing {
//     pub fn share(&self, _secret: i64) -> Vec<i64> {
//         // Sample self.num_shares nuber of random numbers
//         let secret = _secret.rem_euclid(self.prime);
//         return vals;
//     }

//     pub fn reconstruct(&self, shares: &Vec<i64>) -> i64 {
//         let sum: i64 = shares.iter().sum();
//         return sum.rem_euclid(self.prime);
//     }
// }

// pub struct PackedAdditiveSecretSharing {
//     pub num_shares: usize,
//     pub prime: i64,
//     pub dimension: usize,
// }

// impl PackedAdditiveSecretSharing {
//     pub fn share(&self, _secret: i64) -> Vec<i64> {
//         // Sample self.num_shares nuber of random numbers
//         let secret = _secret.rem_euclid(self.prime);
//         let range = Uniform::from(0..self.prime - 1);
//         let mut vals: Vec<i64> = rand::thread_rng()
//             .sample_iter(&range)
//             .take(self.num_shares - 1)
//             .collect();
//         let sum: i64 = vals.iter().sum();
//         let last_share: i64 = (secret - sum).rem_euclid(self.prime);
//         vals.push(last_share);
//         return vals;
//     }

//     pub fn reconstruct(&self, shares: &Vec<i64>) -> i64 {
//         let sum: i64 = shares.iter().sum();
//         return sum.rem_euclid(self.prime);
//     }

//     pub fn packed_share(&self, secrets: &Vec<i64>) -> Vec<Vec<i64>> {
//         let mut packed_shares: Vec<Vec<i64>> = Vec::new();

//         for _secret in secrets {
//             let _secret: i64 = *_secret;
//             let shares: Vec<i64> = self.share(_secret);
//             packed_shares.push(shares);
//         }
//         return packed_shares;
//     }

//     pub fn packed_reconstruct(&self, shares: &Vec<Vec<i64>>) -> Vec<i64> {
//         let mut secrets: Vec<i64> = Vec::new();
//         for _share in shares {
//             let secret = self.reconstruct(_share);
//             secrets.push(secret);
//         }
//         return secrets;
//     }
// }

#[test]
fn test_additive_secret_sharing() {
    let mut commitment = CommitedAdditiveSecretSharing::new(512, 10).unwrap();
    let mut msg1 = 12;
    let mut shares = commitment.share(msg1);
    let mut msg1_hat = commitment.reconstruct(shares);
    assert_eq!(msg1_hat, BigNum::from_u32(msg1).unwrap());

    msg1 = 10231;
    shares = commitment.share(msg1);
    msg1_hat = commitment.reconstruct(shares);
    assert_eq!(msg1_hat, BigNum::from_u32(msg1).unwrap());

}

// #[test]
// fn test_packed_additive_secret_sharing() {
//     let client = PackedAdditiveSecretSharing {
//         num_shares: 10,
//         prime: 41,
//         dimension: 7,
//     };

//     for _ in 1..=100 {
//         let range = Uniform::from(0..client.prime - 1);
//         let random_secrets: Vec<i64> = rand::thread_rng()
//             .sample_iter(&range)
//             .take(client.dimension)
//             .collect();
//         let shares = client.packed_share(&random_secrets);
//         let reconstructed_answer: Vec<i64> = client.packed_reconstruct(&shares);

//         assert_eq!(random_secrets.len(), reconstructed_answer.len());

//         let it = random_secrets.iter().zip(reconstructed_answer.iter());

//         for (_, (x, y)) in it.enumerate() {
//             assert_eq!(*x, *y);
//         }
//     }
// }
