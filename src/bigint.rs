use std::ops::{AddAssign, Mul, SubAssign};

type Digit = usize;
#[cfg(target_pointer_width = "32")]
type WideDigit = u64;
#[cfg(target_pointer_width = "64")]
type WideDigit = u128;

pub struct FixedBigUInt<const N: usize>([Digit; N]);

impl<const N: usize> FixedBigUInt<N> {
    pub const ZERO: Self = Self([0; N]);

    // fn div_rem()
}

impl<const N: usize> AddAssign<&Self> for FixedBigUInt<N> {
    fn add_assign(&mut self, rhs: &Self) {
        add_from(&mut self.0, &rhs.0);
    }
}

impl<const N: usize> SubAssign<&Self> for FixedBigUInt<N> {
    fn sub_assign(&mut self, rhs: &Self) {
        sub_from(&mut self.0, &rhs.0);
    }
}

impl<const N: usize> Mul<Self> for &FixedBigUInt<N> {
    type Output = FixedBigUInt<N>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = FixedBigUInt::ZERO;
        long_fma(&self.0, &rhs.0, &mut res.0);
        res
    }
}

#[inline]
fn add_from(x: &mut [Digit], y: &[Digit]) -> bool {
    let mut carry = false;
    for i in 0..x.len().min(y.len()) {
        let (of1, of2);
        (x[i], of1) = x[i].overflowing_add(y[i]);
        (x[i], of2) = x[i].overflowing_add(carry as Digit);
        carry = of1 | of2;
    }
    for i in y.len()..x.len() {
        if !carry {
            break;
        }
        (x[i], carry) = x[i].overflowing_add(carry as Digit);
    }
    carry
}

#[inline]
fn sub_from(x: &mut [Digit], y: &[Digit]) -> bool {
    let mut borrow = false;
    for i in 0..x.len().min(y.len()) {
        let (of1, of2);
        (x[i], of1) = x[i].overflowing_sub(y[i]);
        (x[i], of2) = x[i].overflowing_sub(borrow as Digit);
        borrow = of1 | of2;
    }
    for i in y.len()..x.len() {
        if !borrow {
            break;
        }
        (x[i], borrow) = x[i].overflowing_sub(borrow as Digit);
    }
    borrow
}

#[inline]
fn add_write(x: &[Digit], y: &[Digit], dst: &mut [Digit]) -> bool {
    let (x, y) = if x.len() >= y.len() { (x, y) } else { (y, x) };
    let mut carry = false;
    for i in 0..y.len().min(dst.len()) {
        dst[i] = x[i];
        let (of1, of2);
        (dst[i], of1) = dst[i].overflowing_add(y[i]);
        (dst[i], of2) = dst[i].overflowing_add(carry as Digit);
        carry = of1 | of2;
    }

    for i in y.len()..x.len().min(dst.len()) {
        if !carry {
            break;
        }
        (dst[i], carry) = x[i].overflowing_add(carry as Digit);
    }
    carry
}

#[inline]
fn long_fma(x: &[Digit], y: &[Digit], dst: &mut [Digit]) {
    for i in 0..x.len().min(dst.len()) {
        let mut carry = false;
        for j in 0..y.len() {
            if i + j >= dst.len() {
                break;
            }
            let prod = x[i] as WideDigit * y[j] as WideDigit + carry as WideDigit;
            if i + j + 1 < dst.len() {
                let wide = dst[i + j] as WideDigit + (dst[i + j + 1] as WideDigit) << Digit::BITS;
                let (sum, of) = prod.overflowing_add(wide);
                dst[i + j] = sum as Digit;
                dst[i + j + 1] = (sum >> Digit::BITS) as Digit;
                carry = of;
            } else {
                dst[i + j] = dst[i + j].wrapping_add(prod as Digit);
            }
        }
    }
}

fn toom3() {
    
}