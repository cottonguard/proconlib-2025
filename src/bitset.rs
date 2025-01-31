use std::fmt::{self, Display};

pub trait BitSet {
    type Chunk;
    fn bit(&self, i: usize) -> bool;
    fn set_bit(&mut self, i: usize, f: bool) -> bool;
    fn flip_bit(&mut self, i: usize) -> bool;
    fn len_bits(&self) -> usize;
    fn count_ones(&self) -> usize;
    fn not(&mut self);
    fn and(&mut self, other: &Self);
    fn or(&mut self, other: &Self);
    fn xor(&mut self, other: &Self);
    fn difference(&mut self, other: &Self);
    fn reverse_bits(&mut self);
    fn display_bits(&self) -> DisplayBits<&Self>;
    fn one_positions(&self) -> OnePositions<&Self>;
}

macro_rules! bitset {
    ($ty:ty, $fmt:literal) => {
        impl BitSet for [$ty] {
            type Chunk = $ty;
            #[inline]
            fn bit(&self, i: usize) -> bool {
                self[i / <$ty>::BITS as usize] >> (i % <$ty>::BITS as usize) & 1 != 0
            }
            #[inline]
            fn set_bit(&mut self, i: usize, f: bool) -> bool {
                let orig = self.bit(i);
                let q = i / <$ty>::BITS as usize;
                let r = i % <$ty>::BITS as usize;
                if f {
                    self[q] |= 1 << r;
                } else {
                    self[q] &= !(1 << r);
                }
                orig
            }
            #[inline]
            fn flip_bit(&mut self, i: usize) -> bool {
                let orig = self.bit(i);
                self[i / <$ty>::BITS as usize] ^= 1 << i % <$ty>::BITS as usize;
                orig
            }
            #[inline]
            fn len_bits(&self) -> usize {
                <$ty>::BITS as usize * self.len()
            }
            #[inline]
            fn count_ones(&self) -> usize {
                self.iter().map(|x| x.count_ones() as usize).sum()
            }
            #[inline]
            fn not(&mut self) {
                for x in self.iter_mut() {
                    *x = !*x;
                }
            }
            #[inline]
            fn and(&mut self, other: &Self) {
                assert_eq!(self.len(), other.len());
                for (x, y) in self.iter_mut().zip(other.iter()) {
                    *x &= y;
                }
            }
            #[inline]
            fn or(&mut self, other: &Self) {
                assert_eq!(self.len(), other.len());
                for (x, y) in self.iter_mut().zip(other.iter()) {
                    *x |= y;
                }
            }
            #[inline]
            fn xor(&mut self, other: &Self) {
                assert_eq!(self.len(), other.len());
                for (x, y) in self.iter_mut().zip(other.iter()) {
                    *x ^= y;
                }
            }
            #[inline]
            fn difference(&mut self, other: &Self) {
                assert_eq!(self.len(), other.len());
                for (x, y) in self.iter_mut().zip(other.iter()) {
                    *x &= !y;
                }
            }
            #[inline]
            fn reverse_bits(&mut self) {
                self.reverse();
                for x in self {
                    *x = x.reverse_bits();
                }
            }
            #[inline]
            fn display_bits(&self) -> DisplayBits<&[$ty]> {
                DisplayBits(self)
            }
            #[inline]
            fn one_positions(&self) -> OnePositions<&Self> {
                OnePositions {
                    data: self,
                    i: 0,
                    j: self.len_bits(),
                }
            }
        }
        impl Iterator for OnePositions<&[$ty]> {
            type Item = usize;
            fn next(&mut self) -> Option<Self::Item> {
                while self.i < self.j {
                    let q = self.i / <$ty>::BITS as usize;
                    let r = self.i % <$ty>::BITS as usize;
                    let masked = self.data[q] & (!0 << r);
                    if masked != 0 {
                        let r = masked.trailing_zeros() as usize;
                        let i = <$ty>::BITS as usize * q + r;
                        self.i = i + 1;
                        return Some(i);
                    }
                    self.i = self.j.min(<$ty>::BITS as usize * (q + 1));
                }
                None
            }
        }
        impl Display for DisplayBits<&[$ty]> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for x in self.0.iter() {
                    write!(f, $fmt, x.reverse_bits())?;
                }
                Ok(())
            }
        }
    };
}

#[cfg(target_pointer_width = "32")]
bitset!(usize, "{:032b}");
#[cfg(target_pointer_width = "64")]
bitset!(usize, "{:064b}");
bitset!(u8, "{:08b}");
bitset!(u16, "{:016b}");
bitset!(u32, "{:032b}");
bitset!(u64, "{:064b}");
bitset!(u128, "{:0128b}");

pub struct DisplayBits<T>(pub T);

pub struct OnePositions<T> {
    data: T,
    i: usize,
    j: usize,
}
