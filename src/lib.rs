#![no_std]
pub mod new_rng;
pub mod old_rng;
pub use old_rng as rng;

#[derive(Debug)]
pub enum SeedInitializer {
    SeedPair(u64, u64),
    Seed(u64),
    Seed0(u64),
    Seed1(u64),
}

impl From<i64> for SeedInitializer {
    fn from(seed: i64) -> Self {
        Self::Seed(seed as u64)
    }
}

impl From<(u64, u64)> for SeedInitializer {
    fn from((seed0, seed1): (u64, u64)) -> Self {
        Self::SeedPair(seed0, seed1)
    }
}

pub const MH3_FACTOR_1: u64 = 0xff51afd7ed558ccd;
pub const MH3_FACTOR_2: u64 = 0xc4ceb9fe1a85ec53;

pub const INV_MH3_FACTOR_1: u64 = 0x9cb4b2f8129337db;
pub const INV_MH3_FACTOR_2: u64 = 0x4f74430c22a54005;

pub trait RandomXS128: 
    From<SeedInitializer>
{
    fn new(seed: u64) -> Self;
    fn next_u64(&mut self) -> u64;
    fn advance(&mut self, n: u32) {
        (0..n).for_each(|_| { let _ = self.next_u64(); })
    }

    fn next_capped_u64(&mut self, modulus: u64) -> u64 {
        loop {
            let (residue, overflowed) = 
                self.overflowing_next_capped_u64(modulus);
            #[cfg(feature = "reroll")]
            if overflowed {
                continue;
            }
            return residue;
        }
    }

    fn unchecked_next_capped_u64(&mut self, modulus: u64) -> u64 {
        self.overflowing_next_capped_u64(modulus).0
    }

    fn overflowing_next_capped_u64(&mut self, modulus: u64) -> (u64, bool);
}

#[cfg(test)]
mod unit_tests {
    enum RngValue {
        U64(u64),
        CappedU64 { modulus: u64, residue: u64 },
        Advance(u32),
    }

    struct RngValues {
        seed: u64,
        values: [Option<RngValue>; 16],
    }

    use crate::{new_rng, old_rng, RandomXS128};

    const VALUE_LISTS: [RngValues; 7] = [
        RngValues {
            seed: 12345i64 as u64,
            values: [ Some(RngValue::U64(1382432690769144372i64 as u64)), Some(RngValue::Advance(10)), Some(RngValue::CappedU64 { modulus: 134, residue: 83, }), Some(RngValue::U64(-5355119237153046436i64 as u64)), None, None, None, None, None, None, None, None, None, None, None, None, ],
        },
        RngValues {
            seed: 42i64 as u64,
            values: [ Some(RngValue::CappedU64 { modulus: 1, residue: 0, }), Some(RngValue::CappedU64 { modulus: 2, residue: 1, }), Some(RngValue::CappedU64 { modulus: 3, residue: 0, }), Some(RngValue::CappedU64 { modulus: 4, residue: 0, }), Some(RngValue::CappedU64 { modulus: 5, residue: 1, }), Some(RngValue::CappedU64 { modulus: 6, residue: 5, }), Some(RngValue::CappedU64 { modulus: 7, residue: 5, }), Some(RngValue::CappedU64 { modulus: 8, residue: 3, }), Some(RngValue::CappedU64 { modulus: 9, residue: 7, }), Some(RngValue::CappedU64 { modulus: 10, residue: 7, }), Some(RngValue::CappedU64 { modulus: 11, residue: 9, }), Some(RngValue::CappedU64 { modulus: 12, residue: 9, }), Some(RngValue::CappedU64 { modulus: 13, residue: 4, }), Some(RngValue::CappedU64 { modulus: 14, residue: 13, }), Some(RngValue::CappedU64 { modulus: 15, residue: 3, }), Some(RngValue::CappedU64 { modulus: 16, residue: 6, }), ],
        },
        RngValues {
            seed: -10i64 as u64,
            values: [ Some(RngValue::CappedU64 { modulus: 5, residue: 3, }), Some(RngValue::CappedU64 { modulus: 2, residue: 0, }), Some(RngValue::CappedU64 { modulus: 9, residue: 7, }), Some(RngValue::Advance(1)), Some(RngValue::U64(-1057127458437580682i64 as u64)), Some(RngValue::CappedU64 { modulus: 7, residue: 6, }), Some(RngValue::CappedU64 { modulus: 13, residue: 3, }), Some(RngValue::CappedU64 { modulus: 8, residue: 0, }), Some(RngValue::CappedU64 { modulus: 6, residue: 0, }), Some(RngValue::CappedU64 { modulus: 4, residue: 0, }), Some(RngValue::CappedU64 { modulus: 3, residue: 1, }), Some(RngValue::CappedU64 { modulus: 11, residue: 3, }), Some(RngValue::CappedU64 { modulus: 16, residue: 7, }), Some(RngValue::CappedU64 { modulus: 12, residue: 7, }), Some(RngValue::CappedU64 { modulus: 15, residue: 8, }), None, ],
        },
        RngValues {
            seed: 1000i64 as u64,
            values: [ Some(RngValue::Advance(2)), Some(RngValue::Advance(3)), Some(RngValue::Advance(4)), Some(RngValue::CappedU64 { modulus: 1, residue: 0, }), Some(RngValue::CappedU64 { modulus: 999, residue: 913, }), Some(RngValue::CappedU64 { modulus: 555, residue: 61, }), Some(RngValue::CappedU64 { modulus: 100, residue: 79, }), Some(RngValue::Advance(5)), Some(RngValue::CappedU64 { modulus: 777, residue: 616, }), Some(RngValue::Advance(6)), Some(RngValue::CappedU64 { modulus: 123, residue: 23, }), Some(RngValue::Advance(7)), Some(RngValue::CappedU64 { modulus: 9999, residue: 7303, }), Some(RngValue::CappedU64 { modulus: 888, residue: 133, }), Some(RngValue::CappedU64 { modulus: 444, residue: 167, }), None, ],
        },
        RngValues {
            seed: 1234567890i64 as u64,
            values: [ Some(RngValue::CappedU64 { modulus: 5, residue: 3, }), Some(RngValue::CappedU64 { modulus: 10, residue: 5, }), Some(RngValue::CappedU64 { modulus: 15, residue: 4, }), Some(RngValue::CappedU64 { modulus: 20, residue: 13, }), Some(RngValue::CappedU64 { modulus: 25, residue: 1, }), Some(RngValue::CappedU64 { modulus: 30, residue: 10, }), Some(RngValue::CappedU64 { modulus: 35, residue: 5, }), Some(RngValue::CappedU64 { modulus: 40, residue: 1, }), Some(RngValue::CappedU64 { modulus: 45, residue: 41, }), Some(RngValue::CappedU64 { modulus: 50, residue: 14, }), Some(RngValue::CappedU64 { modulus: 55, residue: 35, }), Some(RngValue::CappedU64 { modulus: 60, residue: 40, }), Some(RngValue::CappedU64 { modulus: 65, residue: 11, }), Some(RngValue::CappedU64 { modulus: 70, residue: 22, }), Some(RngValue::CappedU64 { modulus: 75, residue: 40, }), Some(RngValue::CappedU64 { modulus: 80, residue: 5, }), ],
        },
        RngValues {
            seed: -9876543210i64 as u64,
            values: [ Some(RngValue::Advance(3)), Some(RngValue::CappedU64 { modulus: 42, residue: 25, }), Some(RngValue::Advance(6)), Some(RngValue::Advance(9)), Some(RngValue::CappedU64 { modulus: 99999999999999, residue: 21627940712847, }), Some(RngValue::CappedU64 { modulus: 77777777777777, residue: 75498257424579, }), Some(RngValue::CappedU64 { modulus: 88888888888888, residue: 14384965901149, }), Some(RngValue::CappedU64 { modulus: 1234567890, residue: 975571841, }), Some(RngValue::Advance(5)), Some(RngValue::CappedU64 { modulus: 55555555555555, residue: 29505003891522, }), Some(RngValue::CappedU64 { modulus: 10000000000000, residue: 551591108261, }), Some(RngValue::CappedU64 { modulus: 123, residue: 15, }), Some(RngValue::Advance(4)), Some(RngValue::CappedU64 { modulus: 99999999999, residue: 52894208431, }), Some(RngValue::Advance(1)), None, ],
        },
        RngValues {
            seed: 9223372036854775807i64 as u64,
            values: [ Some(RngValue::CappedU64 { modulus: 100, residue: 58, }), Some(RngValue::CappedU64 { modulus: 200, residue: 87, }), Some(RngValue::CappedU64 { modulus: 300, residue: 297, }), Some(RngValue::CappedU64 { modulus: 400, residue: 281, }), Some(RngValue::CappedU64 { modulus: 500, residue: 321, }), Some(RngValue::CappedU64 { modulus: 600, residue: 304, }), Some(RngValue::CappedU64 { modulus: 700, residue: 649, }), Some(RngValue::CappedU64 { modulus: 800, residue: 574, }), Some(RngValue::CappedU64 { modulus: 900, residue: 775, }), Some(RngValue::CappedU64 { modulus: 1000, residue: 373, }), Some(RngValue::CappedU64 { modulus: 1100, residue: 465, }), Some(RngValue::CappedU64 { modulus: 1200, residue: 980, }), Some(RngValue::CappedU64 { modulus: 1300, residue: 234, }), Some(RngValue::CappedU64 { modulus: 1400, residue: 917, }), Some(RngValue::CappedU64 { modulus: 1500, residue: 1185, }), None, ],
        },
    ];

    #[test]
    fn old_rng_vs_java_rng() {
        for RngValues { seed, values } in VALUE_LISTS {
            let mut rng = old_rng::Random::new(seed);
            for value in values {
                match value {
                    Some(RngValue::U64(n)) =>
                        assert_eq!(rng.next_u64(), n),
                    Some(RngValue::CappedU64 { modulus, residue }) =>
                        assert_eq!(rng.next_capped_u64(modulus), residue),
                    Some(RngValue::Advance(k)) =>
                        rng.advance(k),
                    None => {}
                }
            }
        }
    }

    #[test]
    fn new_rng_vs_java_rng() {
        for RngValues { seed, values } in VALUE_LISTS {
            let mut rng = new_rng::Random::new(seed);
            for value in values {
                match value {
                    Some(RngValue::U64(n)) => {
                        assert_eq!(rng.next_u64(), n)
                    }
                    Some(RngValue::CappedU64 { modulus, residue }) => {
                        assert_eq!(rng.next_capped_u64(modulus), residue)
                    }
                    Some(RngValue::Advance(k)) => (0..k).for_each(|_| {
                        let _ = rng.next_u64();
                    }),
                    None => {}
                }
            }
        }
    }
}

#[cfg(kani)]
mod verification {

    use crate::old_rng::Random as OldRandom;
    use crate::new_rng::Random as NewRandom;
    use super::*;

    #[kani::proof]
    fn next_u64() {
        let seed0 = kani::any();
        let seed1 = kani::any();
        let mut old_rng: OldRandom = (seed0, seed1).into();
        let mut new_rng: NewRandom = (seed0, seed1).into();
        
        assert!(
            old_rng.next_u64() ==
            new_rng.next_u64()
        );
    }

    #[kani::proof]
    fn next_u64_capped_raw() {
        let seed0 = kani::any();
        let seed1 = kani::any();
        let modulus: u64 = kani::any();


        kani::assume(
            modulus.is_power_of_two() ||
            [3, 5, 6, 7].contains(&modulus)
        );

        let mut old_rng: OldRandom = (seed0, seed1).into();
        let mut new_rng: NewRandom = (seed0, seed1).into();
        
        assert!(
            old_rng.overflowing_next_capped_u64(modulus) ==
            new_rng.overflowing_next_capped_u64(modulus)
        );
    }

    #[kani::proof]
    fn overflowing_next_u64_capped() {
        let seed0 = kani::any();
        let seed1 = kani::any();
        let modulus: u64 = kani::any();


        kani::assume(
            modulus.is_power_of_two()
            // || [3, 5, 6, 7].contains(&modulus)
        );

        let mut old_rng: OldRandom = (seed0, seed1).into();
        let mut new_rng: NewRandom = (seed0, seed1).into();
        
        assert!(
            old_rng.unchecked_next_capped_u64(modulus) ==
            new_rng.unchecked_next_capped_u64(modulus)
        );
    }

    #[kani::proof]
    fn wrapping_xor_shr33() {
        let seed0 = kani::any();
        
        let old_shr33: u64 = seed0 ^ seed0 >> 33;
        let new_shr33: u64 = new_rng::Random
            ::wrapping_xor_shr33(seed0);
        assert!(
            old_shr33 ==
            new_shr33
        );
    }

    // #[kani::proof]
    // fn wrapping_mul() {
    //     let seed0: u64 = kani::any();
        
    //     const FACTORS: [u64; 4] = [
    //         MH3_FACTOR_1,
    //         MH3_FACTOR_2,
    //         INV_MH3_FACTOR_1,
    //         INV_MH3_FACTOR_2
    //     ];
        
    //     for FACTOR in FACTORS {
    //         let old_mul: u64 = seed0.wrapping_mul(FACTOR);
    //         let new_mul: u64 = new_rng::Random
    //             ::wrapping_const_mul::<{FACTOR}>(seed0);
            
    //         assert!(
    //             old_mul ==
    //             new_mul
    //         );
    //     }
    // }
}
