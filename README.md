# Verifiable Differential Privacy

**NOTE:** This is not production ready code, used only for prototyping and generating numbers reported in the paper. To reproduce expriments in the paper see instructions below. 

**Update**: In this repository we simulate inter server communication in a single thread via a for loop. A more practical setting is to follow [this example](https://github.com/henrycg/heavyhitters) and setup multiple servers (and adapt the interfaace). For the purposes of prototyping we did not find this necessary.

## Getting Started 

Make sure you have a working rust and cargo installation

```bash
$ rustc --version
rustc 1.63.0 (4b91a6ea7 2022-08-08)
$ cargo --version
cargo 1.63.0 (fd9c4297c 2022-07-01)
```

## Pedersen Commitments

Pedersen Commitments are the fundamental primitive used to implement further complex primitives in this paper. It is implemented on top of the [Ristretto Curve]() and details about implementation can be found in 

```
src/generic_commitments.rs
```

To see time taken to commit to a random 256 bit integer run the following command

```
cargo run --example commitment --release
```

# Generating Figures In the Paper

## Table I: Benchmarking Components For Each Phase of The Protocol

The experiments in this section could be embarassingly parallelised if needed. Thus the numbers reported here are pessimistic as they are estimates of sequential exeriments. Setting parameters: 
+ the number of clients $n=10^6$, 
+ privacy parameters $\epsilon=0.0.05$ and $\delta$ = 10^-10$, results in $\eta_b = 262144$ private coins.


### Aggregating Finite Field Elements

Time Taken to add 1000000 + 262144 = 1262144 integers;

```
cargo run --example aggregation --release
```

To change the parameters change $n, n_b$ in examples/aggregation.rs


### Proof Creation

```
 cargo run --example fiat_shamir_creation --release
```


### Proof Verification

```
 cargo run --example fiat_shamir_verification --release
```

### Aggregating Group Elements

Time Taken to add 1000000 + 262144 = 1262144 points in the ristretto curve;

```
cargo run --example com_aggregation --release
```

To change the parameters change $n, n_b$ in examples/com_aggregation.rs


## Figure I: Latency vs Privacy Parameter

```
 cargo run --example privacy_vs_latency --release
```

## Figure II: Comparison With Prio And Poplar

**TODO**

