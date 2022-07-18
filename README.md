# Verifiable Differential Privacy with secret sharing

Code for our paper **CITE**.

**NOTE:** This is not production ready code, used only for prototyping and generating numbers reported in the paper. To reproduce expriments in the paper see instructions below.

## Getting Started 

Make sure you have a working rust and cargo installation

```bash
$ rustc --version
rustc 1.59.0 (9d1b2106e 2022-02-23)
$ cargo --version
cargo 1.59.0
```

## Client features

```bash
$ cargo run tests

...

test client::test_voting ... ok
test client::test_bad_input1 - should panic ... ok
test server::test_agg_verify ... ok
test client::test_commitments ... ok
test client::test_bad_commitments - should panic ... ok
```


Shown below is an example of unit test for creating shares for an input

```rust
#[test]
pub fn test_voting(){

    // Parameters 
    let security_parameter = 4; // number bits of security to use
    let num_candidates = 5; // M
    let num_shares = 4; // K
    

    let mut public_param = crate::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap(); 
    let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);

    let vote = 0; // <-- Voting for the first candidate
    let share_of_shares = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers
    for dim in 0..num_candidates{
        let mut recons_share = BigNum::new().unwrap();
        for server_idx in 0..num_shares{
            recons_share = (&recons_share + &share_of_shares[dim].shares[server_idx]).rem(&client.q);
        }
        // Check if the reconstruction is 1 only for voted canidate
        assert_eq!(vote as usize == dim, recons_share == BigNum::from_u32(1).unwrap());
    }
}
```

Shown below is an example of unit test that validates the commitments

```rust

#[test]
pub fn test_commitments(){
    // Parameters 
    let security_parameter = 4; // number bits of security to use
    let num_candidates = 5; // M
    let num_shares = 4; // K

    let mut public_param = crate::public_parameters::PublicParams::new(security_parameter, num_shares).unwrap(); 
    let client = Client::new(num_shares, num_candidates as u32, &public_param.p, &public_param.q, &public_param.g, &public_param.h);

    // Should crash : Client cannot vote for a candidate not in 
    let vote = 0;    
    let share_of_shares = client.vote(vote, &mut public_param.ctx); // there are M commitments for K servers

    for dim in 0..num_candidates{
        for server_idx in 0..num_shares{
            let shr = &share_of_shares[dim].shares[server_idx];
            let rnd = &share_of_shares[dim].randomness[server_idx];
            let com = &share_of_shares[dim].commitments[server_idx];

            assert_eq!(true, client.open(&com, &shr, &rnd, &mut public_param.ctx).unwrap());
        }
    }
    
}
```


## Server side tests

TODO 

## How to generate plots for paper

TODO

## Running a full simulation

```bash
cargo run --example simulation
```

