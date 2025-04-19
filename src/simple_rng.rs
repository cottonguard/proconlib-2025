use std::ops::{Bound, RangeBounds};

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
            .wrapping_mul(0x5851F42D4C957F2D)
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

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        f32::from_bits((127 << 23) | (self.next_u32() & (!0 >> 9))) - 1.0
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        f64::from_bits((1023 << 52) | (self.next_u64() & (!0 >> 12))) - 1.0
    }

    #[inline]
    pub fn random<T: Random>(&mut self) -> T {
        T::random(self)
    }

    #[inline]
    pub fn range<T: Range>(&mut self, range: impl RangeBounds<T>) -> T {
        T::range(self, range)
    }

    #[inline]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for j in (1..slice.len()).rev() {
            let i = self.range(..=j);
            slice.swap(i, j);
        }
    }
}

pub trait Random {
    fn random(rng: &mut Rng) -> Self;
}

pub trait Range {
    fn range(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self;
}

impl Random for u8 {
    fn random(rng: &mut Rng) -> Self {
        rng.next_u32() as _
    }
}
impl Random for u16 {
    fn random(rng: &mut Rng) -> Self {
        rng.next_u32() as _
    }
}
impl Random for u32 {
    fn random(rng: &mut Rng) -> Self {
        rng.next_u32()
    }
}
impl Random for u64 {
    fn random(rng: &mut Rng) -> Self {
        rng.next_u64()
    }
}
impl Random for usize {
    fn random(rng: &mut Rng) -> Self {
        if cfg!(target_pointer_width = "32") {
            rng.next_u32() as usize
        } else if cfg!(target_pointer_width = "64") {
            rng.next_u64() as usize
        } else {
            unimplemented!()
        }
    }
}
macro_rules! random_signed {
    ($ity:ident, $uty:ident) => {
        impl Random for $ity {
            fn random(rng: &mut Rng) -> Self {
                $uty::random(rng) as $ity
            }
        }
    };
}
random_signed!(isize, usize);
random_signed!(i32, u32);
random_signed!(i64, u64);

macro_rules! range {
    ($ty:ident, $uty:ident, $uwide:ident) => {
        impl Range for $ty {
            #[inline]
            fn range(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self {
                let l = match range.start_bound() {
                    Bound::Included(&l) => l,
                    Bound::Excluded(&l) => l + 1,
                    Bound::Unbounded => $ty::MIN,
                };
                let r = match range.end_bound() {
                    Bound::Included(&r) => r.wrapping_add(1),
                    Bound::Excluded(&r) => r,
                    Bound::Unbounded => $ty::MAX.wrapping_add(1),
                };

                assert!(
                    l <= r.wrapping_sub(1),
                    "invalid range ({:?}, {:?})",
                    range.start_bound(),
                    range.end_bound()
                );

                let d = r.wrapping_sub(l) as $uty;
                l.wrapping_add(
                    (($uty::random(rng) as $uwide * d as $uty as $uwide) >> $uty::BITS) as $ty,
                )
            }
        }
    };
}

#[cfg(target_pointer_width = "32")]
range!(usize, u32, u64);
#[cfg(target_pointer_width = "32")]
range!(isize, u32, u64);
#[cfg(target_pointer_width = "64")]
range!(usize, u64, u128);
#[cfg(target_pointer_width = "64")]
range!(isize, u64, u128);
range!(u32, u32, u64);
range!(i32, u32, u64);
range!(u64, u64, u128);
range!(i64, u64, u128);

// c = floor(2^32 / q)
// floor(2^32(a+1) / q) - floor(2^32a / q) = c + 1
// b + fract(2^32x / q)
// b + fract(2^32c / q) < 1
// b < 1 - fract(2^32c / q)
