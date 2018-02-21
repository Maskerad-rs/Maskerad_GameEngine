// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use rand::{random, XorShiftRng, Rand, Rng, SeedableRng};
use rand::distributions::range::SampleRange;

//A wrapper of a Xorshift random number generator.
pub struct RandomNumber {
    generator: XorShiftRng,
}

impl RandomNumber {
    //Generate a new Xorshift generator with random seed.
    pub fn new() -> Self {

        let seed: [u32; 4] = [random(), random(), random(), random()];

        RandomNumber {
            generator: XorShiftRng::from_seed(seed),
        }
    }

    pub fn gen_range<T: PartialOrd + SampleRange>(&mut self, low: T, high: T) -> T {
        self.generator.gen_range(low, high)
    }

    pub fn gen_range_100_int(&mut self) -> u32 {
        self.generator.gen_range::<u32>(0 ,101)
    }

    pub fn gen_range_100_float(&mut self) -> f32 {
        let value = self.generator.gen_range::<f32>(0.0, 100.1);
        if value > 100.0 {100.0} else {value}
    }

    pub fn gen<T: Rand>(&mut self) -> T {
        self.generator.gen()
    }
}

//Those tests are here to assure that the Xorshift rng wrapper works as it should, and is
//sufficiently 'pseudorandom'.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift_gen() {
        let mut rng = RandomNumber::new();
        let value_u32 = rng.gen::<u32>();
        let value_u64 = rng.gen::<u64>();
        let value_i32 = rng.gen::<i32>();
        let value_i64 = rng.gen::<i64>();
        let value_f32 = rng.gen::<f32>();
        let value_f64 = rng.gen::<f64>();

        assert!(value_f32 >= 0.0 && value_f32 < 1.0);
        assert!(value_f64 >= 0.0 && value_f64 < 1.0);

        //Rand for integers : Uniformly distributed over all values of the type.
        //Rand for floats : in range [0; 1)
        for _ in 0..1000 {
            let temp_value_u32 = rng.gen::<u32>();
            assert_ne!(value_u32, temp_value_u32);

            let temp_value_u64 = rng.gen::<u64>();
            assert_ne!(value_u64, temp_value_u64);

            let temp_value_i32 = rng.gen::<i32>();
            assert_ne!(value_i32, temp_value_i32);

            let temp_value_i64 = rng.gen::<i64>();
            assert_ne!(value_i64, temp_value_i64);

            let temp_value_f32 = rng.gen::<f32>();
            assert!(temp_value_f32 >= 0.0 && temp_value_f32 < 1.0);
            assert_ne!(value_f32, temp_value_f32);

            let temp_value_f64 = rng.gen::<f64>();
            assert!(temp_value_f64 >= 0.0 && temp_value_f64 < 1.0);
            assert_ne!(value_f64, temp_value_f64);
        }
    }

    #[test]
    fn xorshift_gen_range() {
        let mut rng = RandomNumber::new();
        //gen range generates a value in the range [low, high)

        let value_u32 = rng.gen_range_100_int();
        assert!(value_u32 <= 100);
        let value_i32 = rng.gen_range::<i32>(-100, 101);
        assert!(value_i32 >= -100 && value_i32 <= 100);
        let value_i64 = rng.gen_range::<i64>(-1000, 1001);
        assert!(value_i64 >= -1000 && value_i64 <= 1000);
        let value_f32 = rng.gen_range_100_float();
        assert!(value_f32 >= 0.0 && value_f32 <= 100.0);
        let value_f64 = rng.gen_range::<f64>(-100.0, 100.0);
        assert!(value_f64 >= -100.0 && value_f64 <= 100.0);

        for _ in 0..100 {
            let temp_value_u32 = rng.gen_range_100_int();
            assert!(temp_value_u32 <= 100);
            let temp_value_i32 = rng.gen_range::<i32>(-100, 101);
            assert!(temp_value_i32 >= -100 && temp_value_i32 <= 100);
            let temp_value_i64 = rng.gen_range::<i64>(-1000, 1001);
            assert!(temp_value_i64 >= -1000 && temp_value_i64 <= 1000);
            let temp_value_f32 = rng.gen_range_100_float();
            assert!(temp_value_f32 >= 0.0 && temp_value_f32 <= 100.0);
            let temp_value_f64 = rng.gen_range::<f64>(-100.0, 100.0);
            assert!(temp_value_f64 >= -100.0 && temp_value_f64 <= 100.0);
        }
    }
}