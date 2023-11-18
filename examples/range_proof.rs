// The #-commented lines are hidden in Rustdoc but not in raw
// markdown rendering, and contain boilerplate code so that the
// code in the README.md is actually run as part of the test suite.
extern crate rand;
use rand::thread_rng;
extern crate curve25519_dalek;
use curve25519_dalek_ng::scalar::Scalar;

extern crate merlin;
use merlin::Transcript;
extern crate bulletproofs;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};

fn main() {
// Generators for Pedersen commitments.  These can be selected
// independently of the Bulletproofs generators.
let pc_gens = PedersenGens::default();

// Generators for Bulletproofs, valid for proofs up to bitsize 64
// and aggregation size up to 1.
let bp_gens = BulletproofGens::new(64, 1);

// A secret value we want to prove lies in the range [0, 2^32)
let secret_value = 10;

// The API takes a blinding factor for the commitment.
let blinding = Scalar::random(&mut thread_rng());

// The proof can be chained to an existing transcript.
// Here we create a transcript with a doctest domain separator.
let mut prover_transcript = Transcript::new(b"doctest example");

// Create a 32-bit rangeproof.
let (proof, committed_value) = RangeProof::prove_single(
    &bp_gens,
    &pc_gens,
    &mut prover_transcript,
    secret_value,
    &blinding,
    32,
).expect("A real program could handle errors");

// Verification requires a transcript with identical initial state:
let mut verifier_transcript = Transcript::new(b"doctest example");
assert!(
    proof
        .verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &committed_value, 32)
        .is_ok()
);
}