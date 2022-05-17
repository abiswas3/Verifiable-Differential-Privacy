extern crate dp_client as ss;


fn main(){
    let client = ss::additive::PackedAdditiveSecretSharing{
        num_shares: 5,
        prime: 41,
        dimension: 3,
    };
    let random_secrets: Vec<i64> = vec![1, 3, 5];
    let shares = client.packed_share(&random_secrets);

    println!("Shares");
    let mut i=0;
    for share in &shares{
        println!("{}: {:?}", random_secrets[i], share);
        i +=1;
    }

    let reconstructed_answer: Vec<i64> = client.packed_reconstruct(&shares);
    let it = random_secrets.iter().zip(reconstructed_answer.iter());

    println!("Check: (orignal: reconstructed)");
    for (_, (x, y)) in it.enumerate() {
        println!("{}:{}", *x,*y);
    }
}