use std::{
    fmt::{self, Debug, Display},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub fn mint<const M: u32>(x: impl Into<ModInt<M>>) -> ModInt<M> {
    x.into()
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModInt<const M: u32>(pub u32);

impl<const M: u32> ModInt<M> {
    pub fn normalize(self) -> Self {
        Self(self.0 % M)
    }
    pub fn get(self) -> u32 {
        self.0
    }
    pub fn get_negative(self) -> i32 {
        if self.0 == 0 {
            0
        } else {
            self.0 as i32 - M as i32
        }
    }
    pub fn inv(self) -> Self {
        use std::mem::swap;
        assert_ne!(self, ModInt(0));
        let mut x = self.0 as i32;
        let mut y = M as i32;
        let mut a = (1, 0);
        let mut b = (0, 1);
        while y != 0 {
            let d = x / y;
            x %= y;
            swap(&mut x, &mut y);
            a.0 -= d * b.0;
            a.1 -= d * b.1;
            swap(&mut a, &mut b);
        }
        debug_assert_eq!(x, 1, "{} (mod {}) does not have inverse", self.0, M);
        Self(if a.0 >= 0 {
            a.0 as u32
        } else {
            (a.0 + M as i32) as u32
        })
    }
    pub const fn primitive_root() -> Self {
        let mut m = M - 1;
        let mut p = 2;
        let mut ds = [0; 32];
        let mut ds_len = 0;
        while p * p <= m {
            if m % p == 0 {
                ds[ds_len] = m / p;
                ds_len += 1;
                m /= p;
                while m % p == 0 {
                    m /= p;
                }
            }
            p += 1;
        }
        let mut r = 2;
        'r: while r < M - 1 {
            let mut i = 0;
            while i < ds_len {
                if Self(r).pow_const(ds[i]).0 == 1 {
                    r += 1;
                    continue 'r;
                }
                i += 1;
            }
            return Self(r);
        }
        panic!("not found primitive root");
    }
    pub(crate) const fn pow_const(self, mut exp: u32) -> Self {
        if exp == 0 {
            return Self(1);
        }
        let mut base = self;
        let mut acc = Self(1);
        loop {
            if exp % 2 == 1 {
                acc = acc.mul_const(base);
                if exp == 1 {
                    return acc;
                }
            }
            base = base.mul_const(base);
            exp /= 2;
        }
    }
    pub(crate) const fn mul_const(self, other: Self) -> Self {
        Self((self.0 as u64 * other.0 as u64 % M as u64) as u32)
    }
}

impl<const M: u32> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(0) - self
    }
}

impl<const M: u32> Neg for &ModInt<M> {
    type Output = ModInt<M>;
    fn neg(self) -> Self::Output {
        -(*self)
    }
}

impl<const M: u32> Add for ModInt<M> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let s = self.0 + other.0;
        Self(if s < M { s } else { s - M })
    }
}

impl<const M: u32> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let (s, of) = self.0.overflowing_sub(other.0);
        Self(if of { s.wrapping_add(M) } else { s })
    }
}

impl<const M: u32> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self((self.0 as u64 * other.0 as u64 % M as u64) as u32)
    }
}

impl<const M: u32> Div for ModInt<M> {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Self) -> Self::Output {
        self * other.inv()
    }
}

macro_rules! ops {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<const M: u32> $Op<ModInt<M>> for &ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, other: ModInt<M>) -> Self::Output {
                (*self).$op(other)
            }
        }
        impl<const M: u32> $Op<&Self> for ModInt<M> {
            type Output = Self;
            fn $op(self, other: &Self) -> Self::Output {
                self.$op(*other)
            }
        }
        impl<const M: u32> $Op for &ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, other: Self) -> Self::Output {
                (*self).$op(*other)
            }
        }
        impl<const M: u32> $OpAssign for ModInt<M> {
            fn $op_assign(&mut self, other: Self) {
                *self = (*self).$op(other);
            }
        }
        impl<const M: u32> $OpAssign<&Self> for ModInt<M> {
            fn $op_assign(&mut self, other: &Self) {
                *self = (*self).$op(other);
            }
        }
    };
}

ops!(Add, add, AddAssign, add_assign);
ops!(Sub, sub, SubAssign, sub_assign);
ops!(Mul, mul, MulAssign, mul_assign);
ops!(Div, div, DivAssign, div_assign);

pub trait Pow<Exp> {
    fn pow(self, exp: Exp) -> Self;
}

macro_rules! pow {
    ($ty:ident, $ity:ident) => {
        impl<const M: u32> Pow<$ty> for ModInt<M> {
            fn pow(self, mut exp: $ty) -> Self {
                if exp == 0 {
                    return Self(1);
                }
                let mut base = self;
                let mut acc = Self(1);
                loop {
                    if exp % 2 == 1 {
                        acc *= base;
                        if exp == 1 {
                            return acc;
                        }
                    }
                    base *= base;
                    exp /= 2;
                }
            }
        }
        impl<const M: u32> Pow<$ity> for ModInt<M> {
            fn pow(mut self, exp: $ity) -> Self {
                if exp < 0 {
                    self = self.inv();
                }
                self.pow(exp.unsigned_abs())
            }
        }
    };
}

pow!(usize, isize);
pow!(u8, i8);
pow!(u16, i16);
pow!(u32, i32);
pow!(u64, i64);
pow!(u128, i128);

impl<const M: u32> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sum: u64 = iter.into_iter().map(|x| x.0 as u64).sum();
        sum.into()
    }
}

impl<const M: u32> Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter().fold(Self(1), |acc, x| acc * x)
    }
}

impl<const M: u32> Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<const M: u32> Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

macro_rules! from_small_uint {
    ($uty:ident, $ity:ident) => {
        impl<const M: u32> From<$uty> for ModInt<M> {
            fn from(x: $uty) -> Self {
                Self(x as u32 % M)
            }
        }
        impl<const M: u32> From<$ity> for ModInt<M> {
            fn from(x: $ity) -> Self {
                Self((x as i32).rem_euclid(M as i32) as u32)
            }
        }
    };
}
macro_rules! from_large_uint {
    ($uty:ident, $ity:ident) => {
        impl<const M: u32> From<$uty> for ModInt<M> {
            fn from(x: $uty) -> Self {
                Self((x % M as $uty) as u32)
            }
        }
        impl<const M: u32> From<$ity> for ModInt<M> {
            fn from(x: $ity) -> Self {
                Self((x.rem_euclid(M as $ity)) as u32)
            }
        }
    };
}

from_small_uint!(u8, i8);
from_small_uint!(u16, i16);
from_large_uint!(u32, i32);
from_large_uint!(u64, i64);
from_large_uint!(u128, i128);
from_large_uint!(usize, isize);
