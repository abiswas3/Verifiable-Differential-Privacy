use optimus_prime::number_theory as nt;
use rand;
use rand::{distributions::Uniform, Rng}; // 0.8.0
pub struct ShamirsSecretSharing{

    pub threshold: usize,
    pub num_shares: usize,
    pub prime: i64,
}

impl ShamirsSecretSharing {
    
    pub fn share(&self, _secret: i64) -> Vec<i64>{
        
        // Sample random polynomial of degree self.threshold + 1
        let poly = self.sample_polynomial(_secret);
        return self.evaluate_polynomial(&poly);
    }

    fn sample_polynomial(&self, _secret: i64)->Vec<i64>{

        let mut coefficients = vec![_secret];
        let range = Uniform::from(0..self.prime-1);
        let vals: Vec<i64> = rand::thread_rng().sample_iter(&range).take(self.threshold).collect();
        coefficients.extend(vals);
        // secret is first element
        return coefficients;
    }

    fn evaluate_polynomial(&self, coefficients: &[i64])-> Vec<i64>{        
        // could fix this to provide points 
        return (1..self.num_shares + 1)
        .map(|point|nt::mod_evaluate_polynomial(coefficients, point as i64, self.prime))
        .collect();
    }
    
    pub fn reconstruct(&self, indices: &[usize], shares: &[i64]) -> i64 {

        assert!(shares.len() == indices.len());
        assert!(shares.len() >= self.threshold + 1);

        // Players are indexed from 0 so adding 1 to indices.
        let points: Vec<i64> = indices.iter().map(|&i| (i as i64) + 1i64).collect();

        return nt::lagrange_interpolation_at_zero(&*points, &shares, self.prime)
;
    }
}