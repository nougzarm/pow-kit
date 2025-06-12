use crate::ProofOfWork;
use rand::Rng;
use sha2::{Digest, Sha256};

/// Implementation of a POW at the Hashcash:
/// Find a nonce such that hash (challenge || nonce) begins with n zeros binary.
pub struct HashcashPoW {
    // number of 0 bit at the beginning of the hash
    pub difficulty: u32,
}

impl ProofOfWork for HashcashPoW {
    type Challenge = String;
    type Solution = u64;

    fn generate_challenge(&self) -> Self::Challenge {
        let mut rng = rand::rng();
        let random_data: u64 = rng.random();
        format!("challenge-{}", random_data)
    }

    fn solve(&self, challenge: &Self::Challenge) -> Self::Solution {
        let mut nonce = 0u64;
        loop {
            if Self::check(challenge, nonce, self.difficulty) {
                return nonce;
            }
            nonce += 1;
        }
    }

    fn verify(&self, challenge: &Self::Challenge, solution: &Self::Solution) -> bool {
        Self::check(challenge, *solution, self.difficulty)
    }
}

impl HashcashPoW {
    fn check(challenge: &str, nonce: u64, difficulty: u32) -> bool {
        let input = format!("{}{}", challenge, nonce);
        let hash = Sha256::digest(input.as_bytes());
        let mut bits = hash
            .iter()
            .flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1))
            .take(difficulty as usize);
        bits.all(|b| b == 0)
    }
}
