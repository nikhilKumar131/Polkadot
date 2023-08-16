#![no_std]

pub struct SimplePrng {
    seed: u64,
    a: u64,
    c: u64,
    m: u64,
}

impl SimplePrng {
    pub fn new(seed: u64) -> Self {
        // Common values for LCG algorithm
        let a = 1664525;
        let c = 1013904223;
        let m = 2u64.pow(32);

        Self { seed, a, c, m }
    }

    pub fn next(&mut self) -> u64 {
        self.seed = (self.a * (self.seed) + (self.c)) % self.m;
        self.seed
    }
}
