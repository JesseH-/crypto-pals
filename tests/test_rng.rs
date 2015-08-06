extern crate cryptopals;
extern crate rand;

use cryptopals::crypto::rng::{MT};
use rand::{Rng, SeedableRng};

#[test]
fn test_rng_deterministic() {
    let mut m1: MT = SeedableRng::from_seed(314159);
    let mut m2: MT = SeedableRng::from_seed(314159);
    for _ in 0 .. 1024 {
        assert_eq!(m1.gen::<u32>(), m2.gen::<u32>());
    }
}
