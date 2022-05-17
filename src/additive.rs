
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
        let last_share: i64 = (secret - sum).rem_euclid(self.prime);
        vals.push(last_share);
        return vals;
    }

    pub fn reconstruct(&self, shares: &Vec<i64>)->i64{
        let sum: i64 = shares.iter().sum();
        return sum.rem_euclid(self.prime);
    }
}

pub struct PackedAdditiveSecretSharing{

    pub num_shares: usize,
    pub prime: i64,
    pub dimension: usize,
}

impl PackedAdditiveSecretSharing{

    pub fn share(&self, _secret: i64) -> Vec<i64>{
        // Sample self.num_shares nuber of random numbers
        let secret = _secret.rem_euclid(self.prime);
        let range = Uniform::from(0..self.prime-1);
        let mut vals: Vec<i64> = rand::thread_rng().sample_iter(&range).take(self.num_shares-1).collect();
        let sum: i64 = vals.iter().sum();
        let last_share: i64 = (secret - sum).rem_euclid(self.prime);
        vals.push(last_share);
        return vals;
    }

    pub fn reconstruct(&self, shares: &Vec<i64>)->i64{
        let sum: i64 = shares.iter().sum();
        return sum.rem_euclid(self.prime);
    }

    pub fn packed_share(&self, secrets: &Vec<i64>) -> Vec<Vec<i64>>{
        let mut packed_shares: Vec<Vec<i64>> = Vec::new();

        for _secret in secrets{
            let _secret: i64 = *_secret;
            let shares: Vec<i64> = self.share(_secret);
            packed_shares.push(shares);
        }
        return packed_shares;
    }

    pub fn packed_reconstruct(&self, shares: &Vec<Vec<i64>>)->Vec<i64>{

        let mut secrets: Vec<i64> = Vec::new();
        for _share in shares{
            let secret = self.reconstruct(_share);
            secrets.push(secret);
        }
        return secrets;
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
        assert_eq!(secret.rem_euclid(client.prime), client.reconstruct(&shares));    
    }
}

#[test]
fn test_packed_additive_secret_sharing(){

    let client = PackedAdditiveSecretSharing{
        num_shares: 10,
        prime: 41,
        dimension: 7,
    };

    
    for _ in 1..=100{
        let range = Uniform::from(0..client.prime-1);
        let random_secrets: Vec<i64> = rand::thread_rng().sample_iter(&range).take(client.dimension).collect();
        let shares = client.packed_share(&random_secrets);
        let reconstructed_answer: Vec<i64> = client.packed_reconstruct(&shares);

        assert_eq!(random_secrets.len(), reconstructed_answer.len());

        let it = random_secrets.iter().zip(reconstructed_answer.iter());

        for (_, (x, y)) in it.enumerate() {
            assert_eq!(*x,*y);
        }   
    }


}