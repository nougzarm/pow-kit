use crate::{HashcashPoW, ProofOfWork};

/// This part tests the functioning of HashcashPOW
#[allow(non_snake_case)]
#[test]
fn hashcash_test() {
    let pow = HashcashPoW { difficulty: 20 };

    let challenge = pow.generate_challenge();
    println!("Challenge: {}", challenge);

    let solution = pow.solve(&challenge);
    println!("Found nonce: {}", solution);

    let is_valid = pow.verify(&challenge, &solution);
    assert!(is_valid);
}
