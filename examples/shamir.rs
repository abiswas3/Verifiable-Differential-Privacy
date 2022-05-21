extern crate dp_client as ss;
fn main(){

    let client = ss::shamir::ShamirsSecretSharing{
        num_shares: 5,
        prime: 41,
        threshold: 3,
    };

    let secret: i64 = (198 as i64).rem_euclid(client.prime);
    let all_shares  = client.share(secret);
    
    let indices: Vec<usize> = (0..client.threshold+1).collect();
    let shares: &[i64] = &all_shares[0..client.threshold+1];
    let recovered_secret = client.reconstruct(&indices, shares);

    println!("Secret:{} The recovered secret is {}", secret, recovered_secret);}