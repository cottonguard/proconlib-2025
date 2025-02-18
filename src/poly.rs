use std::{
    ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
    slice::SliceIndex,
};

use crate::modint::*;

#[derive(Clone, Default, Debug)]
pub struct Poly<const M: u32>(pub Vec<ModInt<M>>);

impl<const M: u32> Poly<M> {
    pub fn deg(&self) -> isize {
        self.0
            .iter()
            .rposition(|a| a.get() != 0)
            .map(|deg| deg as isize)
            .unwrap_or(-1)
    }
    pub fn normalize(&mut self) {
        let n = self
            .0
            .iter()
            .rposition(|a| a.get() != 0)
            .map(|i| i + 1)
            .unwrap_or(0);
        self.truncate(n);
    }
    pub(crate) fn dft_mul(&mut self, other: &mut Self) {
        if self.is_empty() || other.is_empty() {
            self.clear();
            return;
        }
        let len = (self.len() + other.len() - 1).next_power_of_two();
        self.resize(len, ModInt(0));
        dft(self);
        other.resize(len, ModInt(0));
        dft(other);
        for (a, b) in self.iter_mut().zip(other.iter()) {
            *a *= b;
        }
        idft(self);
        self.normalize();
    }
    fn add_assign_impl(&mut self, other: &Self, op: impl Fn(ModInt<M>, ModInt<M>) -> ModInt<M>) {
        if other.len() > self.len() {
            self.resize(other.len(), ModInt(0));
        }
        for (a, b) in self.iter_mut().zip(other.iter()) {
            *a = op(*a, *b);
        }
    }
    fn add_impl(&self, other: &Self, op: impl Fn(ModInt<M>, ModInt<M>) -> ModInt<M>) -> Self {
        let (f, g) = if self.len() >= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        let mut res = Poly(vec![ModInt(0); f.len()]);
        for i in 0..res.len() {
            res[i] = if i < g.len() { op(f[i], g[i]) } else { f[i] }
        }
        res
    }
    pub fn dft(&mut self) {
        dft(&mut self.0);
    }
    pub fn idft(&mut self) {
        idft(&mut self.0);
    }
    pub fn inv(&self, mod_deg: usize) -> Self {
        assert!(!self.is_empty());
        let mut f = Self(vec![]);
        let mut inv = Self(vec![self[0].inv()]);
        while inv.len() < mod_deg {
            let len = inv.len();
            inv.resize(4 * len, ModInt(0));
            inv.dft();
            f.clear();
            f.extend_from_slice(&self[..(2 * len).min(self.len())]);
            f.resize(4 * len, ModInt(0));
            f.dft();
            for i in 0..4 * len {
                inv[i] = inv[i] * inv[i] * f[i];
            }
            inv.idft();
            inv.truncate(2 * len);
            for a in &mut inv[len..] {
                *a = -*a;
            }
        }
        inv.truncate(mod_deg);
        inv.normalize();
        inv
    }
}

impl<const M: u32> Neg for Poly<M> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for a in self.iter_mut() {
            *a = -*a;
        }
        self
    }
}

impl<const M: u32> AddAssign<&Self> for Poly<M> {
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign_impl(rhs, |x, y| x + y);
    }
}

impl<const M: u32> Add for &Poly<M> {
    type Output = Poly<M>;
    fn add(self, rhs: Self) -> Self::Output {
        self.add_impl(rhs, |x, y| x + y)
    }
}

impl<const M: u32> SubAssign<&Self> for Poly<M> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.add_assign_impl(rhs, |x, y| x - y);
    }
}

impl<const M: u32> Sub for &Poly<M> {
    type Output = Poly<M>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.add_impl(rhs, |x, y| x - y)
    }
}

impl<const M: u32> Mul for Poly<M> {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const M: u32> MulAssign for Poly<M> {
    fn mul_assign(&mut self, mut rhs: Self) {
        self.dft_mul(&mut rhs);
    }
}

impl<const M: u32> Deref for Poly<M> {
    type Target = Vec<ModInt<M>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const M: u32> DerefMut for Poly<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<I: SliceIndex<[ModInt<M>]>, const M: u32> Index<I> for Poly<M> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}
impl<I: SliceIndex<[ModInt<M>]>, const M: u32> IndexMut<I> for Poly<M> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

fn dft<const M: u32>(a: &mut Vec<ModInt<M>>) {
    dft_impl::<false, M>(a);
}

fn idft<const M: u32>(a: &mut Vec<ModInt<M>>) {
    dft_impl::<true, M>(a);
}

#[inline]
fn dft_impl<const INV: bool, const M: u32>(a: &mut Vec<ModInt<M>>) {
    if a.len() <= 1 {
        return;
    }
    a.resize(a.len().next_power_of_two(), ModInt(0));
    let shift = a.len().leading_zeros() + 1;
    for i in 0..a.len() {
        let j = i.reverse_bits() >> shift;
        if i < j {
            a.swap(i, j);
        }
    }
    dft_rec::<INV, M>(a);
    if INV {
        let d = ModInt(a.len() as u32).inv();
        for a in a.iter_mut() {
            *a *= d;
        }
    }
}

fn dft_rec<const INV: bool, const M: u32>(a: &mut [ModInt<M>]) {
    if a.len() <= 1 {
        return;
    }
    let exp = if INV {
        M - 1 - (M - 1) / a.len() as u32
    } else {
        (M - 1) / a.len() as u32
    };
    let w_base = ModInt::primitive_root().pow_const(exp);
    let h = a.len() / 2;
    dft_rec::<INV, M>(&mut a[..h]);
    dft_rec::<INV, M>(&mut a[h..]);
    let mut w = ModInt(1);
    for i in 0..h {
        let p = a[i];
        let q = w * a[i + h];
        a[i] = p + q;
        a[i + h] = p - q;
        w *= w_base;
    }
}

/*
a <- a + w_n/2^i b
b <- a - w_n/2^i b
c <- c + w_n/2^i d
d <- c - w_n/2^i d

a <- a + w_n^i c
c <- a - w_n^i c
b <- b + w_n^i w_4 d
d <- b - w_n^i w_4 d

a <- a + w_n/2^i b + w_n^i(c + w_n/2^i d)
   = a + w_n^2i b + w_n^i c + w_n3i d
c <- a + w_n^2i b - w_n^i c - w_n3i d
b <- a - w_n^2i b + w_4(w_n^i c - w_n^3i d)
d <- a - w_n^2i b - w_4(w_n^i c - w_n^3i d)
*/
mod tmp {
    #![allow(unused)]
    use super::*;
    fn dft4_rec<const INV: bool, const M: u32>(a: &mut [ModInt<M>]) {
        if a.len() <= 1 {
            return;
        }
        let exp = if INV {
            M - 1 - (M - 1) / a.len() as u32
        } else {
            (M - 1) / a.len() as u32
        };
        let w_base = ModInt::primitive_root().pow_const(exp);
        let wi = if INV {
            ModInt::primitive_root().pow_const(M - 1 - (M - 1) / 4)
        } else {
            ModInt::primitive_root().pow_const((M - 1) / 4)
        };
        let q = a.len() / 4;
        dft_rec::<INV, M>(&mut a[..q]);
        dft_rec::<INV, M>(&mut a[q..2 * q]);
        dft_rec::<INV, M>(&mut a[2 * q..3 * q]);
        dft_rec::<INV, M>(&mut a[3 * q..]);
        let mut w = ModInt(1);
        for i in 0..q {
            let w2 = w * w;
            let w3 = w2 * w;
            let a0 = a[i];
            let a1 = w2 * a[i + q];
            let a2 = w * a[i + 2 * q];
            let a3 = w3 * a[i + 3 * q];
            let s01 = a0 + a1;
            let d01 = a0 - a1;
            let s23 = a2 + a3;
            let d23i = wi * (a2 - a3);
            a[i] = s01 + s23;
            a[i + q] = d01 + d23i;
            a[i + 2 * q] = s01 - s23;
            a[i + 3 * q] = d01 - d23i;
            w *= w_base;
        }
    }

    /*
    00000000
    00004444
    00442266
    00446622
    04261537
    04265173
    04623715
    04627351
    */
    fn dft8<const M: u32>(a: &mut [ModInt<M>]) {
        let w1 = ModInt::primitive_root().pow_const((M - 1) / 8);
        let w2 = w1 * w1;
        let a0p1 = a[0] + a[1];
        let a0m1 = a[0] - a[1];
        let a2p3 = a[2] + a[3];
        let a2m3 = a[2] - a[3];
        let a2m3w2 = a2m3 * w2;
        let b0 = a0p1 + a2p3;
        let b1 = a0p1 - a2p3;
        let b2 = a0m1 + a2m3w2;
        let b3 = a0m1 - a2m3w2;
        let a4p5 = a[4] + a[5];
        let a4m5 = a[4] - a[5];
        let a4p5w2 = a4p5 * w2;
        let a4m5w1 = a4m5 * w1;
        let a4m5w3 = a4m5w1 * w2;
        let a6p7 = a[6] + a[7];
        let a6p7w2 = a6p7 * w2;
        let a6m7 = a[6] - a[7];
        let a6m7w1 = a6m7 * w1;
        let a6m7w3 = a6m7w1 * w2;
        let c0 = a4p5 + a6p7;
        let c1 = a4p5w2 - a6p7w2;
        let c2 = a4m5w1 + a6m7w3;
        let c3 = a4m5w3 + a6m7w1;
        a[0] = b0 + c0;
        a[1] = b0 - c0;
        a[2] = b1 + c1;
        a[3] = b1 - c1;
        a[4] = b2 + c2;
        a[5] = b2 - c2;
        a[6] = b3 + c3;
        a[7] = b3 - c3;
    }

    fn dft4<const M: u32>(a: &mut [ModInt<M>]) {
        let w = ModInt::primitive_root().pow((M - 1) / 4);
        let a0p1 = a[0] + a[1];
        let a0m1 = a[0] - a[1];
        let a2p3 = a[2] + a[3];
        let a2m3w = (a[2] - a[3]) * w;
        a[0] = a0p1 + a2p3;
        a[1] = a0p1 - a2p3;
        a[2] = a0m1 + a2m3w;
        a[3] = a0m1 - a2m3w;
    }
}
