use CommitedAdditiveSecretSharing;

pub struct Client{
    vote: u32,
    engine: CommitedAdditiveSecretSharing, // This is like the software they use to create shares
}

impl Client{

    pub fn cast_vote(&self){
        
        // Check if vote is well formed
        // This is detectable without any security as length 
        // of vector will change.
        if(self.vote > engine.num_shares){
            
        }
    }
}
