extern crate dp_client as ss;


fn main(){
    let client = ss::additive::PackedAdditiveSecretSharing{
        num_shares: 5,
        prime: 41,
        dimension: 3,
    };
    

    let random_secrets: Vec<i64> = vec![1, 3, 5];
    let shares = client.packed_share(&random_secrets);

    for share in &shares{
        println!("{:?}", share);
    }

    let reconstructed_answer: Vec<i64> = client.packed_reconstruct(&shares);
    let it = random_secrets.iter().zip(reconstructed_answer.iter());

    for (_, (x, y)) in it.enumerate() {
        println!("{}:{}", *x,*y);
    }
}