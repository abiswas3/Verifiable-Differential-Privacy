use openssl::bn::{BigNum, BigNumContext};
use std::ops::Rem;
use std::fmt;
use crate::utils::{gen_random};

pub struct Server{
    pub q: BigNum, // order of diffie Hellman group G
    pub num_candidates: usize,
    pub num_servers: usize,    
}

// NOTE: 
// https://stackoverflow.com/questions/27589054/what-is-the-correct-way-to-use-lifetimes-with-a-struct-in-rust
// Good discussion about lifetimes of references
pub struct Share{
    pub shares: Vec<BigNum>,
}

impl fmt::Display for  Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Server")
            .field("q", &self.q)
            .finish()
    }
}

impl Server{

    pub fn new(num_servers:usize, num_candidates: usize, _q: &BigNum) -> Server {
       
        let q = &BigNum::new().unwrap() + _q;
        Self{q, num_candidates, num_servers}
    }

    pub fn multiply_first(&self, x_i: &BigNum, y_i: &BigNum, a_i: &BigNum, b_i: &BigNum, ctx: &mut BigNumContext)->(BigNum, BigNum){

        let mut d_i = BigNum::new().unwrap();
        let mut e_i = BigNum::new().unwrap();

        _ = d_i.mod_sub(x_i, a_i, &self.q, ctx);
        _ = e_i.mod_sub(y_i, b_i, &self.q, ctx);

        return (d_i, e_i);
    }
    pub fn multiply_second(&self, a_i: &BigNum, b_i: &BigNum, c_i: &BigNum, e: &BigNum, d: &BigNum)->BigNum{

        let db = d*b_i;
        let ea = e*a_i;
        
        
        let xy_i = &(&(&db + &ea) + c_i);
        return xy_i.rem(&self.q);

    }

    pub fn generate_fresh_randomness(&self)->Vec<BigNum>{

        // Randomness to verify if an input is legal or not
        let mut r_vec = Vec::new();
        // For each dim get randomness that is shared by all servers
        for _ in 0..self.num_candidates{
            r_vec.push(gen_random(&self.q).unwrap());
        }
        return r_vec;
    }

    pub fn round_one_verification(&self, v: &Vec<BigNum>, kv: &Vec<BigNum>, randomness: &Vec<BigNum>, a_i: &BigNum, b_i: &BigNum, c_i: &BigNum, ctx: &mut BigNumContext)->(BigNum, BigNum, BigNum){

        // let z = (z_i.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q); 
        // let z_star = (z_i_star.iter().fold(BigNum::from_u32(0).unwrap(), |acc, x| &acc + x)).rem(&self.q);
        let mut z = BigNum::new().unwrap();        
        let mut tmp = BigNum::new().unwrap();
        for i in 0..self.num_candidates{
            _ = tmp.mod_mul(&v[i], &randomness[i], &self.q, ctx);
            z = (&z + &tmp).rem(&self.q);
        }

        let mut z_star = BigNum::new().unwrap();
        tmp = BigNum::new().unwrap();        
        for i in 0..self.num_candidates{
            _ = tmp.mod_mul(&v[i], &(&randomness[i] * &randomness[i]), &self.q, ctx);
            z_star = (&z_star + &tmp).rem(&self.q);
        }

        let mut z_star_star = BigNum::new().unwrap();
        tmp = BigNum::new().unwrap();        
        for i in 0..self.num_candidates{
            _ = tmp.mod_mul(&kv[i], &randomness[i], &self.q, ctx);
            z_star_star = (&z_star_star + &tmp).rem(&self.q);
        }

        return ((&z + a_i).rem(&self.q), (&z_star + b_i).rem(&self.q), (&z_star_star + c_i).rem(&self.q))

    }

    pub fn round_two_verification(&self, ucase_a_i: &BigNum, ucase_b_i: &BigNum, ucase_z: &BigNum)->BigNum{

        // a_iZ + b_i
        return (&(ucase_a_i*ucase_z) + ucase_b_i).rem(&self.q);

        
    }

}

#[test]
fn test_sketching(){

    // use crate::utils::{gen_random, additive_share};
    use crate::public_parameters::PublicParams;
    use crate::poplar::Server;
    use crate::verifiable_client::Client;
    use crate::beaver_triple::BeaverTriple;

    let security_parameter = 4;
    let num_candidates = 2; // Doesn't play a role in this test but need it to initialise server
    let num_shares = 3; // num_servers
    let mut public_param = PublicParams::new(security_parameter, num_shares).unwrap();
    println!("{}", public_param);

    let client = Client::new(num_shares, num_candidates as u32,  &public_param.q);    
    let choice = client.generate_fake_vote();       
    let vote = client.vote(choice, &mut public_param.ctx);
    let kvote = client.poplar_vote(choice, &mut public_param.ctx);

    let mut servers = Vec::new();
    for _ in 0..num_shares{
        let server = Server::new(num_shares, num_candidates,  &public_param.q);
        servers.push(server);
    }

    // Pre-processing: Client has a,b and c
    let beaver_triples = BeaverTriple::new(num_shares, &public_param.q, &mut public_param.ctx);
    let randomness = servers[0].generate_fresh_randomness();
    
    // Servers never see a, b, c in plain
    let a = beaver_triples.a_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
    let b = beaver_triples.b_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);
    let c = beaver_triples.c_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x);

    // A and B as defined in Appendix C of Poplar
    let (ucase_a_share, ucase_b_share) = client.get_ab_for_poplar(&a, &b, &c, &mut public_param.ctx);

    // ROUND 1 of MPC
    let mut ucase_z = BigNum::new().unwrap();
    let mut ucase_z_star = BigNum::new().unwrap();
    let mut ucase_z_star_star = BigNum::new().unwrap();
    for server_idx in 0..num_shares{
        let mut shares = Vec::new(); // Get the shares for server i (copy it over)
        let mut k_shares =  Vec::new(); 
        for coord in 0..num_candidates{
            shares.push(&vote[coord].shares[server_idx] + &BigNum::new().unwrap());
            k_shares.push(&kvote[coord].shares[server_idx] + &BigNum::new().unwrap());
        }
        let (z, z_star, z_star_star) = servers[server_idx].round_one_verification(&shares, &k_shares, &randomness, &beaver_triples.a_shares[server_idx], &beaver_triples.b_shares[server_idx], &beaver_triples.c_shares[server_idx], &mut public_param.ctx);
        ucase_z = (&ucase_z + &z).rem(&public_param.q);
        ucase_z_star = (&ucase_z_star + &z_star).rem(&public_param.q);
        ucase_z_star_star = (&ucase_z_star_star + &z_star_star).rem(&public_param.q);
    }

    // ROUND 2 of MPC
    let mut ans = BigNum::new().unwrap();
    for server_idx in 0..num_shares{
        let shr_output = servers[server_idx].round_two_verification(&ucase_a_share.shares[server_idx], &ucase_b_share.shares[server_idx], &ucase_z);
        ans = (&ans + &shr_output).rem(&public_param.q);
    }

    let pos = (&ans + &(&ucase_z*&ucase_z)).rem(&public_param.q);
    let neg = &ucase_z_star + &ucase_z_star_star;
    let mut output = BigNum::new().unwrap();
    _ = output.mod_sub(&pos, &neg, &public_param.q, &mut public_param.ctx);
    assert_eq!(output, BigNum::new().unwrap());


}