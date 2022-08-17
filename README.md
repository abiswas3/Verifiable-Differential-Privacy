# Verifiable Differential Privacy with secret sharing

Update: Make sure you have openssl installed.



**NOTE:** This is not production ready code, used only for prototyping and generating numbers reported in the paper. To reproduce expriments in the paper see instructions below.

**Attention**: In this repository we simulate inter server communication in a single thread via a for loop. A more practical setting is to follow [this example](https://github.com/henrycg/heavyhitters) and setup multiple servers (and adapt the interfaace). For the purposes of prototyping we did not find this necessary.

**Attention**: In this repository we use finite field discrete logarithm. It is advisable to use an elliptic curve implementation instead. It does not effect the correctness of the claims in the paper however. In the coming future we will transition into using ECC.

**Attention**: The experiments below have a dependency on openssl as we use openssls big integer support to perform finite field operations. 


## Getting Started 

Make sure you have a working rust and cargo installation

```bash
$ rustc --version
rustc 1.59.0 (9d1b2106e 2022-02-23)
$ cargo --version
cargo 1.59.0
```
## How to generate plots for paper

For all plots and figures, run the following code as described below which generates the raw data which can then be visualised using the [jupyter notebook](https://github.com/abiswas3/Verifiable-Differential-Privacy/blob/main/scripts_for_paper/PlotsForPaper.ipynb) in scripts directory.

### Secret Sharing

```bash
cargo run --example secret_sharing_comparison
```

### Verifification of inputs

```bash
cargo run --example coms_input_verify
cargo run --example no_coms_input_verify
```


## How to create secret shares

Shown below are code samples of how to create the input shares with commitments. 

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


## Running just the verifiable histogram protocol

An MPC protocol to get K servers to compute plurality election. As long as one server is honest, no client or server can cheat by tampering with the output, tampering with the inputs of the protocol or deviating in any way from the specs.

```bash
cargo run --example no_dp_simulation
```

## Running just the binomial mechanism without client inputs

An MPC protocol to get K servers to generate binomial noise verifiably. If any server deviates from instructions the honest server aborts (Panic's in this case and ends the program).

```bash
cargo run --example binomial_mechanism
```

## Running just the verifiable histogram protocol with DP

An MPC protocol to get K servers to compute DP plurality election. As long as one server is honest, no client or server can cheat by tampering with the output, tampering with the inputs of the protocol or deviating in any way from the specs. Furthermore, as long as the honest server does not adopt we can be certain that the output utility holds.

```bash
cargo run --example full_simulation
```

