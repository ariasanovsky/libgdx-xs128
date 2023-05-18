use super::*;

impl From<i64> for Random {
    fn from(value: i64) -> Self {
        SeedInitializer::Seed(value as u64).into()
    }
}

impl From<(u64, u64)> for Random {
    fn from((seed0, seed1): (u64, u64)) -> Self {
        Random { seed0, seed1 }
    }
}

impl From<SeedInitializer> for Random {
    fn from(value: SeedInitializer) -> Self {
        use SeedInitializer::*;
        match value {
            SeedPair(seed0, seed1) => Random { seed0, seed1 },
            Seed(seed) => {
                let seed0 = Self::murmur_hash3(seed);
                Seed0(seed0).into()
            }
            Seed0(seed0) => {
                let seed1 = Self::murmur_hash3(seed0);
                SeedPair(seed0, seed1).into()
            }
            Seed1(seed1) => {
                let seed0 = Self::inverse_murmur_hash3(seed1);
                SeedPair(seed0, seed1).into()
            }
        }
    }
}
