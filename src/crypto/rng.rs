extern crate rand;

use self::rand::{Rng, SeedableRng};

const W: u32 = 32;
const N: usize = 624;
const M: usize = 397;
const R: u32 = 31;
const A: u32 = 0x9908B0DF;
const U: u32 = 11;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;
const L: u32 = 18;
const F: u32 = 1812433253;

const LM: u32 = (1 << R) - 1;
const UM: u32 = !LM;

pub struct MT {
    pub state: [u32; N],
    pub index: usize
}

impl MT {
    fn twist(&mut self) {
        for i in 0 .. N {
            let x = (self.state[i] & UM) + (self.state[(i + 1) % N] & LM);
            let mut x_a = x >> 1;
            if (x % 2) > 0 { x_a = x_a ^ A }
            self.state[i] = self.state[(i + M) % N] ^ x_a;
        }
        self.index = 0;
    }
}

impl Default for MT {
    fn default() -> MT {
        MT { state: [0; N], index: N }
    }
}

impl Rng for MT {
    fn next_u32(&mut self) -> u32 {
        if self.index == N { self.twist() }

        let mut y = self.state[self.index];
        y ^= y >> U;
        y ^= (y << S) & B;
        y ^= (y << T) & C;
        y ^= y >> L;

        self.index += 1;
        y
    }
}

impl SeedableRng<u32> for MT {
    fn from_seed(seed: u32) -> MT {
        let mut rng = MT { ..Default::default() };
        rng.reseed(seed);
        rng
    }

    fn reseed(&mut self, seed: u32) {
        self.index = N;
        self.state[0] = seed;
        for i in 1 .. N {
            self.state[i] = F.wrapping_mul(
                self.state[i - 1] ^ (self.state[i - 1] >> (W - 2))) + i as u32;
        }
    }
}

fn undo_right_shift_xor(x: u32, shift: u32) -> u32 {
    let mut y = 0;
    for i in 0 .. 32 {
        y |= ((y >> shift) ^ x) & (1 << (31 - i));
    }
    y
}

fn undo_left_shift_mask_xor(x: u32, shift: u32, mask: u32) -> u32 {
    let mut y = 0;
    for i in 0 .. 32 {
        y |= (((y << shift) & mask) ^ x) & (1 << i);
    }
    y
}

pub fn untemper(output: u32) -> u32 {
    let mut state = output;
    state = undo_right_shift_xor(state, L);
    state = undo_left_shift_mask_xor(state, T, C);
    state = undo_left_shift_mask_xor(state, S, B);
    state = undo_right_shift_xor(state, U);
    state
}
