//This 'random' struct is a pseudorandom number generator using the Xorshift random number generator
//created by George Marsaglia, which also created the Mother-Of-All random number generator,
//rivalizing the Mersenne Twister algoritm in term of speed, randomness and ease of implementation
//(https://en.wikipedia.org/wiki/George_Marsaglia)
//(https://en.wikipedia.org/wiki/Xorshift)
//(https://en.wikipedia.org/wiki/Mersenne_Twister)
use rand::{thread_rng, XorShiftRng, Rand, Rng, SeedableRng};
use rand::distributions::range::SampleRange;

//A wrapper of a Xorshift random number generator.
pub struct RandomNumber {
    generator: XorShiftRng,
}

impl RandomNumber {
    //Generate a new Xorshift generator with random seed.
    pub fn new() -> Self {

        let seed: [u32; 4] = [thread_rng().gen(), thread_rng().gen(), thread_rng().gen(), thread_rng().gen()];

        RandomNumber {
            generator: XorShiftRng::from_seed(seed),
        }
    }

    //Reseed the Xorshift generator
    pub fn reseed(&mut self, seed: [u32; 4]) {
        self.generator = XorShiftRng::from_seed(seed);
    }

    pub fn gen_range<T: PartialOrd + SampleRange>(&mut self, low: T, high: T) -> T {
        self.generator.gen_range(low, high)
    }

    pub fn gen<T: Rand>(&mut self) -> T {
        self.generator.gen()
    }

    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.generator.fill_bytes(dest);
    }
}

//Those tests are here to assure that the Xorshift rng wrapper works as it should, and is
//sufficiently 'pseudorandom'.
#[cfg(test)]
mod tests {
    use super::*;


}