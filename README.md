# Verifiable Differential Privacy

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
## Schnorr-Sigma OR Proof

A 3 message interactive ZK proof that allows a prover to convince an efficient verifier that given $c = Com(x, r)$, $c$ is a commitment to $x=1$ or $x=0$.

To see how to create such a proof and then its verification, run 

```
cargo run --example schnorr
```

**NOTE**: The textbook definition of a sigma protocol is **only Honest Verifier Zero-Knowledge**. It can be rendered non-interactive using the Fiat Shamir transform, which makes it ZK in the ROM model. However, the OR protocol can be made fully ZK even in the plain model.

An interactive standard sigma protocol can be made fully zero-knowledge by sampling the challenge from a set $S$ that is polynomial in the size of the security parameter. See Theorem 2 of [Maurer](https://crypto.ethz.ch/publications/files/Maurer09.pdf). A proof sketch of the simulator is provided below. A natural consequence of shrinking the challenge space is that soundness is non-negligble, but by sequential repetition, we can drive the soundness to be negligible as well.

Alternatively the 3 message sigma protocol can be extended to a [4 message protocol](https://link.springer.com/chapter/10.1007/3-540-45539-6_30) which requires the verifier to commit to their challenge before seeing the provers first message, using trapodoor commitments. [This talk by Benny Pinkas](https://youtu.be/m-NW75E8JIE) provides a full description of the protocol and a simulator proof. Not only is this proof fully ZK, it can be also used concurrently.

### Maurer's Proof of Theorem 2 

In Mauer's proof sketch for Theorem 2, he is suggesting that the simulator repeatedly choose the challenge $c$ at random, and use "c-simulatability" to generate a transcript $(a, c, z)$ distributed identically to those generated by the honest prover interacting with the verifier when the challenge is $c$.

With this procedure, the probability that the simulator picks a $c$ such that the dishonest verifier V would have actually sent $c$ in response to the first-message being $a$, and such that V would have accepted $(a, c, z)$ is:

$$ \star = \sum_{c in S} \Pr[\text{simulator picks challenge c}] \times q(c)$$


where $q(c)$ equals: 


$\Pr[\tex{the malicious V winds up sending challenge c when interacting with honest prover, and accepting the resulting transcript}]$.


For at least one $c^\star$, $q(c^\star)$ must be non-negligible. This is because the probability of the dishonest V accepting when interacting with the honest prover is $\sum_{c in S} q(c)$

This is negligible if $q(c)$ is negligible for all $c$, and $|S|$ is polynomial.


Hence, $(\star)$ is at least $(1/|S|) \times q(c^\star)$, which is non-negligible since $|S|$ is polynomial and $q(c^\star)$ is non-negligible. 