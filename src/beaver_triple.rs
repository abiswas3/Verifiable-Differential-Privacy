use std::ops::Rem;
use openssl::bn::{BigNum, BigNumContext};
use crate::utils::{gen_random, additive_share};

pub struct BeaverTriple{
    pub a_shares: Vec<BigNum>,
    pub b_shares: Vec<BigNum>,
    pub c_shares: Vec<BigNum>,
}

impl BeaverTriple{

    pub fn new(num_shares: usize, q: &BigNum, ctx: &mut BigNumContext)-> BeaverTriple{

        let a = gen_random(q).unwrap();
        let b = gen_random(q).unwrap();
        let c = (&a * &b).rem(q);

        let a_shares = additive_share(&a, q, num_shares, ctx);
        let total = a_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x).rem(q);
        assert_eq!(total, a);
    
        let b_shares = additive_share(&b, q, num_shares, ctx);
        let total = b_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x).rem(q);
        assert_eq!(total, b);

        let c_shares = additive_share(&c, q, num_shares, ctx);
        let total = c_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x).rem(q);
        assert_eq!(total, c);

        Self{a_shares, b_shares, c_shares}
    }

}

#[test]
fn test_beaver_mult(){

    use crate::utils::{gen_random, additive_share};
    use crate::public_parameters::PublicParams;
    use crate::prio::Server;

    let security_parameter = 4;
    let num_candidates = 2; // Doesn't play a role in this test but need it to initialise server
    let num_shares = 2; // num_servers
    let mut public_param = PublicParams::new(security_parameter, num_shares).unwrap();
    println!("{}", public_param);

    let server1 = Server::new(num_shares, num_candidates, &public_param.p, &public_param.q, &public_param.g, &public_param.h);

    let beaver_triples = BeaverTriple::new(num_shares, &public_param.q, &mut public_param.ctx);
    let a_shares = beaver_triples.a_shares;
    let b_shares = beaver_triples.b_shares;
    let c_shares = beaver_triples.c_shares;


    let x = gen_random(&public_param.q).unwrap();
    let y = &x + &BigNum::new().unwrap();

    let x_shares = additive_share(&x, &public_param.q, num_shares, &mut public_param.ctx);
    let total = x_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x).rem(&public_param.q);
    assert_eq!(total, x);

    let y_shares = additive_share(&y, &public_param.q, num_shares, &mut public_param.ctx);
    let total = y_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x).rem(&public_param.q);
    assert_eq!(total, y);

    let a = a_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x).rem(&public_param.q);
    let b = b_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x).rem(&public_param.q);
    // let c = c_shares.iter().fold(BigNum::new().unwrap(), |acc, x| &acc + x).rem(&public_param.q);

    let mut d = BigNum::new().unwrap();
    let mut e = BigNum::new().unwrap();
    for i in 0..num_shares{
        let (d_i, e_i) = server1.multiply_first(&x_shares[i], &y_shares[i], &a_shares[i], &b_shares[i],  &mut public_param.ctx);
        d = (&d + &d_i).rem(&public_param.q);
        e = (&e + &e_i).rem(&public_param.q);
    }
    
    let mut tmp = BigNum::new().unwrap();
    _ = tmp.mod_sub(&x, &a, &public_param.q, &mut public_param.ctx);
    assert_eq!(d, tmp);

    tmp = BigNum::new().unwrap();
    _ = tmp.mod_sub(&y, &b, &public_param.q, &mut public_param.ctx);
    assert_eq!(e, tmp);

    println!("x, y: {},{}", x, y);
    let mut ans = BigNum::new().unwrap();
    let de = &d*&e;
    for i in 0..num_shares{
        let output_i = server1.multiply_second(&a_shares[i], &b_shares[i], &c_shares[i], &e, &d);
        println!("{}", output_i);
        ans = &ans + &output_i;
    }
    
    ans = &ans + &de;
    assert_eq!(ans.rem(&public_param.q), (&x*&y).rem(&public_param.q));

}