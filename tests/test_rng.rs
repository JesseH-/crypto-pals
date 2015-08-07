extern crate cryptopals;
extern crate rand;
extern crate time;

use cryptopals::crypto::rng::{MT, untemper};
use rand::{Rng, SeedableRng, thread_rng};
use time::{get_time};

#[test]
fn test_rng_deterministic() {
    let mut m1: MT = SeedableRng::from_seed(314159);
    let mut m2: MT = SeedableRng::from_seed(314159);
    for _ in 0 .. 1024 {
        assert_eq!(m1.gen::<u32>(), m2.gen::<u32>());
    }
}

#[test]
fn test_seed_recovery_from_time() {
    let mut time = get_time().sec;
    time += thread_rng().gen_range(40, 1000);
    let mut m: MT = SeedableRng::from_seed(time as u32);
    let output = m.gen::<u32>();
    for seed in get_time().sec + 2000 .. 0 {
        let mut checker: MT = SeedableRng::from_seed(seed as u32);
        if checker.gen::<u32>() == output {
            assert_eq!(seed, time);
            break;
        }
    }
}

#[test]
fn test_untemper() {
    let mut m: MT = SeedableRng::from_seed(314159);
    for i in 0 .. 624 {
        let output = m.gen::<u32>();
        assert_eq!(untemper(output), m.state[i]);
    }
}

#[test]
fn test_rng_clone_from_output() {
    let mut m: MT = SeedableRng::from_seed(314159);
    let mut state = [0; 624];
    for i in 0 .. 624 {
        state[i] = untemper(m.gen::<u32>());
    }
    let mut cloned = MT { state: state, index: 624 };
    for _ in 0 .. 1024 {
        assert_eq!(cloned.gen::<u32>(), m.gen::<u32>());
    }
}
