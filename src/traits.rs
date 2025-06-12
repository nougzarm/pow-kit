//! [`ProofOfWork`] Trait.
//!
//! This module defines the [`ProofOfWork`] trait, a generic interface for proof of work protocols.

pub trait ProofOfWork {
    type Challenge;
    type Solution;

    fn generate_challenge(&self) -> Self::Challenge;

    fn solve(&self, challenge: &Self::Challenge) -> Self::Solution;

    fn verify(&self, challenge: &Self::Challenge, solution: &Self::Solution) -> bool;
}
