use openssl::bn::{BigNum, BigNumContext};
use sha3::{Digest, Sha3_256};
use std::ops::Rem;
pub struct Board {
    pub coms: Vec<BigNum>, // You need to reconstruct the coms from this
    pub p: BigNum,
    pub q: BigNum,
    pub g: BigNum,
    pub h: BigNum,
    pub e0: BigNum,
    pub e1: BigNum,
    pub e: BigNum,
    pub v0: BigNum,
    pub v1: BigNum,
    pub d0: BigNum,
    pub d1: BigNum,
    pub num_shares: usize,
}

impl Board {
    pub fn new(
        _coms: &Vec<BigNum>,
        _p: &BigNum,
        _q: &BigNum,
        _g: &BigNum,
        _h: &BigNum,
        _e0: &BigNum,
        _e1: &BigNum,
        _e: &BigNum,
        _v0: &BigNum,
        _v1: &BigNum,
        _d0: &BigNum,
        _d1: &BigNum,
        num_shares: usize,
    ) -> Board {
        let p = &BigNum::new().unwrap() + _p;
        let q = &BigNum::new().unwrap() + _q;
        let g = &BigNum::new().unwrap() + _g;
        let h = &BigNum::new().unwrap() + _h;

        let e0 = &BigNum::new().unwrap() + _e0;
        let e1 = &BigNum::new().unwrap() + _e1;
        let e = &BigNum::new().unwrap() + _e;

        let v0 = &BigNum::new().unwrap() + _v0;
        let v1 = &BigNum::new().unwrap() + _v1;
        let d0 = &BigNum::new().unwrap() + _d0;
        let d1 = &BigNum::new().unwrap() + _d1;

        let mut coms = Vec::new();
        for i in 0..num_shares {
            coms.push(&BigNum::new().unwrap() + &_coms[i]);
        }
        Self {
            coms,
            p,
            q,
            g,
            h,
            e0,
            e1,
            e,
            v0,
            v1,
            d0,
            d1,
            num_shares,
        }
    }
    pub fn verify(&self, ctx: &mut BigNumContext) -> bool {
        let mut tmp = BigNum::new().unwrap();
        let mut tmp2 = BigNum::new().unwrap();

        let mut recons_com = BigNum::from_u32(1).unwrap();
        for server_idx in 0..self.num_shares {
            recons_com = &(&recons_com * &self.coms[server_idx]) % &self.p;
        }

        // println!("Inside Audit:\t{}", recons_com);

        let mut v_hasher = Sha3_256::new();
        let mut input_to_rom = (&BigNum::new().unwrap() + &recons_com).to_vec();
        input_to_rom.append(&mut self.d0.to_vec());
        input_to_rom.append(&mut self.d1.to_vec());
        v_hasher.update(input_to_rom); // this will take d0, d1, commitment as a byte array
        let result = v_hasher.finalize();
        let e_retrieved = BigNum::from_slice(&result).unwrap().rem(&self.q);
        assert_eq!(e_retrieved, self.e); // Check if ROM was correctly used

        assert_eq!(self.e, &(&self.e0 + &self.e1) % &self.q); // CHECK e = e0 + e1

        _ = tmp.mod_exp(&recons_com, &self.e0, &self.p, ctx); //c^{e0}
        _ = tmp2.mod_exp(&self.h, &self.v0, &self.p, ctx); //h^{v0}
        assert_eq!(&(&self.d0 * &tmp) % &self.p, tmp2); //d0 c^{e0} = h^{v0}

        _ = tmp.mod_exp(&recons_com, &self.e1, &self.p, ctx); // c^{e1}

        let mut tmp3 = BigNum::new().unwrap();
        _ = tmp3.mod_exp(&self.g, &self.e1, &self.p, ctx); // g^{e1}
        _ = tmp2.mod_exp(&self.h, &self.v1, &self.p, ctx); // h^{v1}

        assert_eq!(&(&self.d1 * &tmp) % &self.p, &(&tmp3 * &tmp2) % &self.p); //d1 c^{e1} = g^{e1}h^{v1}

        return true;
    }
}
