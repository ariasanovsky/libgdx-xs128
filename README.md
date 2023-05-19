# libgdx-xs128

A minimalist replication of `RandomXS128` from the Java library [libgdx](https://github.com/libgdx/libgdx). Implementation inspired by [sts_map_oracle](https://github.com/Ru5ty0ne/sts_map_oracle/).
Unit tests generated in [Java](https://github.com/ariasanovsky/rng-java-app).

## Features

- Generates pseudo-random numbers using a [128-bit Xorshift algorithm](https://en.wikipedia.org/wiki/Xorshift).
- (in progress): enforce `no_panic` on `new_rng` module
- (if requested): Implement the `RngCore` and `SeedableRng` traits from the `rand_core` crate for compatibility with other Rust crates that use the same trait.

## Model-checking with `kani`

We verify partial equivalence of the modules `old_rng` and `new_rng` with the `kani` model checker ([Github](https://github.com/model-checking/kani), [site](https://github.com/model-checking/kani), [crates.io](https://crates.io/crates/kani-verifier), [lib.rs](https://lib.rs/crates/kani-verifier)).
This helps us refactor `new_rng` to always compile without a single `panic`.

## Panic-free progress

```mermaid
graph LR;
    
    classDef Complete stroke:#27ae60, fill:#000, color:#fff, stroke-width:2px;
    classDef NoPanic stroke:#f1c40f, fill:#000, color:#fff, stroke-width:2px;
    classDef Panics stroke:#e74c3c, fill:#000, color:#fff, stroke-width:2px;

    Green[green: model-checked]:::Complete
    --required for-->
    Orange[compiles with #no_panic]:::NoPanic
    -->
    Red[does does compile with #no_panic]:::Panics
    ;
    
    From_u64_u64:::Panics;
    new:::Panics;
    From_SeedInitializer:::Panics;
    murmur_hash3:::Panics;
    inverse_murmur_hash3:::Panics;
    shr_33:::Panics;
    wrapping_const_mul:::Panics;
    From_i64:::Panics;

    new --i64 as u64--> From_i64;
    From_SeedInitializer --Seed--> new;

    From_u64_u64 --SeedPair--> From_SeedInitializer;
    
    murmur_hash3 --Seed & Seed0--> From_SeedInitializer;
    inverse_murmur_hash3 --Seed1--> From_SeedInitializer;


    shr_33 --> murmur_hash3;
    shr_33 --> inverse_murmur_hash3;
    
    wrapping_const_mul --> murmur_hash3;
    wrapping_const_mul --> inverse_murmur_hash3;

    next_u64:::Panics;
    overflowing_next_capped_u64:::Panics;
    advance:::Panics;
    unchecked_next_capped_u64:::Panics;
    next_capped_u64:::Panics;

    next_u64 --> overflowing_next_capped_u64;
    next_u64 --> advance;
    overflowing_next_capped_u64 --> unchecked_next_capped_u64;
    overflowing_next_capped_u64 --> next_capped_u64;
```


## License

Dual-licensed to be compatible with the `Rust` project.

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option.
This file may not be copied, modified, or distributed except according to those terms.
