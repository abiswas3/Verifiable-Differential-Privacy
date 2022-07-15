use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use dp_client::additive::AdditiveSecretSharing;
extern crate dp_client as ss;

pub fn criterion_benchmark(c: &mut Criterion) {
    // let client = ss::additive::PackedAdditiveSecretSharing{
    //     num_shares: 5,
    //     prime: 41,
    //     dimension: 3,
    // };
    // let random_secrets: Vec<i64> = vec![1, 3, 5];

    // c.bench_function("additive sec", |b| b.iter(|| client.packed_share(black_box(&random_secrets))));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);