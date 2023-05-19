mod from;
mod xs128;
pub use from::*;
pub use xs128::*;

use crate::{SeedInitializer, RandomXS128, MH3_FACTOR_1, MH3_FACTOR_2, INV_MH3_FACTOR_1, INV_MH3_FACTOR_2};

#[derive(Debug)]
pub struct Random {
    seed0: u64,
    seed1: u64,
}

impl RandomXS128 for Random {
    fn new(seed: u64) -> Self {
        #[cfg(feature = "check_zero_seed")]
        let seed = if seed == 0 { i64::MIN as u64 } else { seed };
        SeedInitializer::Seed(seed).into()
    }

    fn next_u64(&mut self) -> u64 {
        let mut s1 = self.seed0;
        let s0 = self.seed1;
        self.seed0 = s0;
        s1 ^= s1 << 23;
        self.seed1 = s1 ^ s0 ^ s1 >> 17 ^ s0 >> 26;
        s0.wrapping_add(self.seed1)
    }

    fn overflowing_next_capped_u64(&mut self, modulus: u64) -> (u64, bool)  {
        let bits = self.next_u64().wrapping_shr(1);
        let residue = bits % modulus;
        (residue, bits.wrapping_add(modulus) < residue.wrapping_add(1))
    }
}

impl Random {
    pub(crate) fn wrapping_xor_shr33(x: u64) -> u64 {
        x ^ x.wrapping_shr(33)
    }

    pub(crate) fn wrapping_const_mul<const FACTOR: u64>(x: u64) -> u64 {
        x.wrapping_mul(FACTOR)
    }
    
    pub(crate) fn _wrapping_shr33_and_mult(x: u64, factor: u64) -> u64 {
        let x = Random::wrapping_xor_shr33(x);
        x.wrapping_mul(factor)
    }
    
    pub(crate) fn murmur_hash3(x: u64) -> u64 {
        let mut x = Random::wrapping_xor_shr33(x);
        x = Random::wrapping_const_mul::<MH3_FACTOR_1>(x);
        x = Random::wrapping_xor_shr33(x);
        x = x.wrapping_mul(MH3_FACTOR_2);
        Random::wrapping_xor_shr33(x)
    }

    pub(crate) fn inverse_murmur_hash3(x: u64) -> u64 {
        let mut x = Random::wrapping_xor_shr33(x);
        x = x.wrapping_mul(INV_MH3_FACTOR_1);
        x = Random::wrapping_xor_shr33(x);
        x = x.wrapping_mul(INV_MH3_FACTOR_2);
        Random::wrapping_xor_shr33(x)
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
