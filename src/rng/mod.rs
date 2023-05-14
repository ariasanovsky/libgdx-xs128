#[derive(Debug)]
pub struct Random {
    seed0: u64,
    seed1: u64,
}

#[derive(Debug)]
pub enum SeedInitializer {
    SeedPair(u64, u64),
    Seed(u64), Seed0(u64), Seed1(u64),
}

impl From<SeedInitializer> for Random {
    fn from(value: SeedInitializer) -> Self {
        use SeedInitializer::*;
        match value {
            SeedPair(seed0, seed1) => {
                Random { seed0, seed1 }
            },
            Seed(seed) => {
                let seed0 = Self::murmur_hash3(seed);
                Seed0(seed0).into()
            },
            Seed0(seed0) => {
                let seed1 = Self::murmur_hash3(seed0);
                SeedPair(seed0, seed1).into()
            },
            Seed1(seed1) => {
                let seed0 = Self::inverse_murmur_hash3(seed1);
                SeedPair(seed0, seed1).into()
            },
        }
    }
}

impl Random {
    pub fn new(seed: u64) -> Self {
        #[cfg(feature = "check_zero_seed")]
        let seed = if seed == 0 {
            i64::MIN as u64
        } else { seed };
        SeedInitializer::Seed(seed).into()
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut s1 = self.seed0;
        let s0 = self.seed1;
        self.seed0 = s0;
        s1 ^= s1 << 23;
        self.seed1 = s1 ^ s0 ^ s1 >> 17 ^ s0 >> 26;
        s0.wrapping_add(self.seed1)
    }

    pub fn next_capped_u64(&mut self, modulus: u64) -> u64 {
        loop {
            let bits = self.next_u64() >> 1;
            let residue = bits % modulus;
            #[cfg(feature = "reroll")]
            if bits + modulus < residue + 1 {
                continue
            }
            return residue
        }
    }

    fn murmur_hash3(mut x: u64) -> u64 {
        x ^= x >> 33;
        x = x.wrapping_mul(0xff51afd7ed558ccd);
        x ^= x >> 33;
        x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
        x ^= x >> 33;
        x
    }

    fn inverse_murmur_hash3(mut x: u64) -> u64 {
        x ^= x >> 33;
        x = x.wrapping_mul(0x9cb4b2f8129337db);
        x ^= x >> 33;
        x = x.wrapping_mul(0x4f74430c22a54005);
        x ^= x >> 33;
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inverse() { // O(1) with release flag
        for i in 1..100_000_000 {
            let hashed = Random::murmur_hash3(i);
            let double_hashed = Random::inverse_murmur_hash3(hashed);
            assert_eq!(i, double_hashed)
        }
    }
}