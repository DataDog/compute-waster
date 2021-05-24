// Taken from the rust `rand` crate. See https://github.com/rust-random/rand/blob/master/LICENSE-MIT for license

use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    fn from_seed(seed: [u8; 32]) -> Self {
        if seed.iter().all(|&x| x == 0) {
            return Self::seed_from_u64(0);
        }
        let mut state = [0; 4];
        read_u64_into(&seed, &mut state);
        Self { s: state }
    }

    pub fn seed_from_u64(mut state: u64) -> Self {
        const PHI: u64 = 0x9e3779b97f4a7c15;
        let mut seed = [0; 32];
        for chunk in seed.as_mut().chunks_mut(8) {
            state = state.wrapping_add(PHI);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z = z ^ (z >> 31);
            chunk.copy_from_slice(&z.to_le_bytes());
        }
        Self::from_seed(seed)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result_plusplus = self.s[0]
            .wrapping_add(self.s[3])
            .rotate_left(23)
            .wrapping_add(self.s[0]);

        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;

        self.s[3] = self.s[3].rotate_left(45);

        result_plusplus
    }

    #[inline]
    pub fn next_idx(&mut self, size: usize) -> usize {
        loop {
            let next = self.next_u64() as usize;
            if next < size * ((u64::MAX as usize) / size) {
                break next % size;
            }
        }
    }
}

fn read_u64_into(src: &[u8], dst: &mut [u64]) {
    assert!(src.len() >= 8 * dst.len());
    for (out, chunk) in dst.iter_mut().zip(src.chunks_exact(8)) {
        *out = u64::from_le_bytes(chunk.try_into().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference() {
        let mut rng = Rng::from_seed([
            1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro256plusplus.c
        let expected = [
            41943041,
            58720359,
            3588806011781223,
            3591011842654386,
            9228616714210784205,
            9973669472204895162,
            14011001112246962877,
            12406186145184390807,
            15849039046786891736,
            10450023813501588000,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
