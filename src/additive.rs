
use rand;
use rand::{distributions::Uniform, Rng}; // 0.8.0

// use rand::distributions::{Distribution, Uniform};

pub struct AdditiveSecretSharing{

    pub num_shares: usize,
    pub prime: i64,
}

impl AdditiveSecretSharing {

    pub fn share(&self, _secret: i64) -> Vec<i64>{
        // Sample self.num_shares nuber of random numbers
        let secret = _secret.rem_euclid(self.prime);
        let range = Uniform::from(0..self.prime-1);
        let mut vals: Vec<i64> = rand::thread_rng().sample_iter(&range).take(self.num_shares-1).collect();
        let sum: i64 = vals.iter().sum();
        let last_share: i64 = (secret - sum) % self.prime;
        vals.push(last_share);
        return vals;
    }

    pub fn reconstruct(&self, shares: Vec<i64>)->i64{
        let sum: i64 = shares.iter().sum();
        return sum.rem_euclid(self.prime);
    }
}

#[test]
fn test_additive_secret_sharing(){
    let client = AdditiveSecretSharing{
        num_shares :10,
        prime :41,
    };
    let mut rng = rand::thread_rng();

    for _ in 1..=100{
        let secret: i64 = rng.gen_range(0..100);
        let shares = client.share(secret);
        assert_eq!(secret.rem_euclid(client.prime), client.reconstruct(shares));    
    }
}