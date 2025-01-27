pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut rng = Self(seed.wrapping_add(0xa02bdbf7bb3c0a7));
        rng.step();
        rng
    }

    #[inline]
    fn step(&mut self) {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(0xa02bdbf7bb3c0a7);
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let x = self.0;
        self.step();
        ((((x >> 18) ^ x) >> 27) as u32).rotate_right((x >> 59) as u32)
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.next_u32() as u64 | (self.next_u32() as u64) << 32
    }
}
