# Verifiable Differential Privacy with secret sharing

Update: Make sure you have openssl installed.

**NOTE:** This is not production ready code, used only for prototyping and generating numbers reported in the paper. To reproduce expriments in the paper see instructions below.

**Update**: In this repository we simulate inter server communication in a single thread via a for loop. A more practical setting is to follow [this example](https://github.com/henrycg/heavyhitters) and setup multiple servers (and adapt the interfaace). For the purposes of prototyping we did not find this necessary.

**Update**: The experiments below have a dependency on openssl as we use openssls big integer support to perform finite field operations. 


## Getting Started 

Make sure you have a working rust and cargo installation

```bash
$ rustc --version
rustc 1.59.0 (9d1b2106e 2022-02-23)
$ cargo --version
cargo 1.59.0
```
## How to generate plots for paper

To generate numbers for Table 1, run 
```
cargo run --example counting_query
```

To generate numbers for Figure 4, run 

```
cargo run --example prio_simulation
cargo run --example poplar_simulation
```
