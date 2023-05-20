mod from;
mod xs128;
pub use from::*;
pub use xs128::*;

use crate::{SeedInitializer, RandomXS128, RandomXS128Initialization};

#[derive(Debug)]
pub struct Random {
    seed0: u64,
    seed1: u64,
}

impl Random {
    unsafe fn swap_seeds(&mut self) {
        core::ptr::swap(&mut self.seed0, &mut self.seed1)
    }
    
    pub fn next(&mut self) {
        unsafe { self.swap_seeds(); }
        let s1 = self.seed1 ^ self.seed1.wrapping_shl(23);
        self.seed1 = 
            s1 ^ s1.wrapping_shr(17) ^
            self.seed0 ^ self.seed0.wrapping_shr(26)
        ;
    }

    pub fn current_u64(&self) -> u64 {
        self.seed0.wrapping_add(self.seed1)
    }
}

impl RandomXS128 for Random {
    fn new(seed: u64) -> Self {
        #[cfg(feature = "check_zero_seed")]
        let seed = if seed == 0 { i64::MIN as u64 } else { seed };
        SeedInitializer::Seed(seed).into()
    }

    fn next_u64(&mut self) -> u64 {
        self.next();
        self.current_u64()
    }

    fn unchecked_next_capped_u64(&mut self, modulus: u64) -> u64 {
        self.next_u64().wrapping_shr(1).wrapping_rem(modulus)
    }
    
    fn overflowing_next_capped_u64(&mut self, modulus: u64) -> (u64, bool)  {
        let bits = self.next_u64().wrapping_shr(1);
        let residue = bits.wrapping_rem(modulus);
        (residue, bits.wrapping_add(modulus) < residue.wrapping_add(1))
    }
}

impl RandomXS128Initialization for Random {
    fn wrapping_xor_shr33(x: u64) -> u64 {
        x ^ x.wrapping_shr(33)
    }

    fn wrapping_const_mul<const FACTOR: u64>(x: u64) -> u64 {
        x.wrapping_mul(FACTOR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inverse() {
        // O(1) with release flag
        for i in 1..100_000_000 {
            let hashed = Random::murmur_hash3(i);
            let double_hashed = Random::inverse_murmur_hash3(hashed);
            assert_eq!(i, double_hashed)
        }
    }
}
