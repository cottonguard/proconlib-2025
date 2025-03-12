use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
    ptr, slice,
};

pub trait Scalar:
    Copy + PartialEq + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! prim {
    ($ty:ident) => {
        impl Scalar for $ty {
            const ZERO: Self = 0 as Self;
            const ONE: Self = 1 as Self;
        }
    };
}

prim!(isize);
prim!(i8);
prim!(i16);
prim!(i32);
prim!(i64);
prim!(i128);
prim!(f32);
prim!(f64);

pub trait Vector<T: Scalar> {
    fn size(&self) -> usize;
    fn elem(&self, i: usize) -> T;
    fn elem_mut(&mut self, i: usize) -> &mut T;
    fn set_elem(&mut self, i: usize, e: T) {
        *self.elem_mut(i) = e;
    }
    fn scale(&mut self, a: T) {
        for i in 0..self.size() {
            let e = self.elem_mut(i);
            *e = a * *e;
        }
    }
    fn vec_add<U: Vector<T> + ?Sized, V: Vector<T> + ?Sized>(&mut self, x: &U, y: &V) {
        vec_binop(x, y, self, |x, y| x + y);
    }
    fn vec_sub<U: Vector<T> + ?Sized, V: Vector<T> + ?Sized>(&mut self, x: &U, y: &V) {
        vec_binop(x, y, self, |x, y| x - y);
    }
    fn vec_add_assign<U: Vector<T> + ?Sized>(&mut self, other: &U) {
        vec_binop_assign(self, other, |x, y| x + y);
    }
    fn vec_sub_assign<U: Vector<T> + ?Sized>(&mut self, other: &U) {
        vec_binop_assign(self, other, |x, y| x - y);
    }
    fn dot<U: Vector<T> + ?Sized>(&self, other: &U) -> T {
        assert_eq!(self.size(), other.size());
        (0..self.size())
            .map(|i| self.elem(i) * other.elem(i))
            .fold(T::ZERO, |acc, prod| acc + prod)
    }
    fn norm(&self) -> T {
        self.dot(self)
    }
}

impl<T: Scalar> Vector<T> for [T] {
    fn size(&self) -> usize {
        self.len()
    }
    fn elem(&self, i: usize) -> T {
        self[i]
    }
    fn elem_mut(&mut self, i: usize) -> &mut T {
        &mut self[i]
    }
}

impl<T: Scalar, const N: usize> Vector<T> for [T; N] {
    fn size(&self) -> usize {
        self.len()
    }
    fn elem(&self, i: usize) -> T {
        self[i]
    }
    fn elem_mut(&mut self, i: usize) -> &mut T {
        &mut self[i]
    }
}

impl<T: Scalar> Vector<T> for Vec<T> {
    fn size(&self) -> usize {
        (**self).size()
    }
    fn elem(&self, i: usize) -> T {
        (**self).elem(i)
    }
    fn elem_mut(&mut self, i: usize) -> &mut T {
        (**self).elem_mut(i)
    }
}

fn vec_binop<T: Scalar, V: Vector<T> + ?Sized, U: Vector<T> + ?Sized, W: Vector<T> + ?Sized>(
    x: &V,
    y: &U,
    z: &mut W,
    f: impl Fn(T, T) -> T,
) {
    assert_eq!(x.size(), y.size());
    assert_eq!(x.size(), z.size());
    for i in 0..x.size() {
        z.set_elem(i, f(x.elem(i), y.elem(i)));
    }
}

fn vec_binop_assign<T: Scalar, V: Vector<T> + ?Sized, U: Vector<T> + ?Sized>(
    x: &mut V,
    y: &U,
    f: impl Fn(T, T) -> T,
) {
    assert_eq!(x.size(), y.size());
    for i in 0..x.size() {
        let e = x.elem_mut(i);
        *e = f(*e, y.elem(i));
    }
}

pub trait Matrix<T: Scalar> {
    fn n(&self) -> usize;
    fn m(&self) -> usize;
    fn row(&self, i: usize) -> &[T];
    fn row_mut(&mut self, i: usize) -> &mut [T];
    fn row2_mut(&mut self, i1: usize, i2: usize) -> (&mut [T], &mut [T]);

    fn size(&self) -> (usize, usize) {
        (self.n(), self.m())
    }
    fn elem(&self, i: usize, j: usize) -> T {
        self.row(i)[j]
    }
    fn elem_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.row_mut(i)[j]
    }
    fn set_elem(&mut self, i: usize, j: usize, x: T) {
        *self.elem_mut(i, j) = x;
    }

    fn is_square(&self) -> bool {
        self.n() == self.m()
    }
    fn swap_elem(&mut self, i1: usize, j1: usize, i2: usize, j2: usize) {
        if (i1, j1) == (i2, j2) {
            return;
        }
        let e1 = self.elem(i1, j1);
        let e2 = self.elem(i2, j2);
        self.set_elem(i1, j1, e2);
        self.set_elem(i2, j2, e1);
    }
    fn swap_row(&mut self, i1: usize, i2: usize) {
        if i1 == i2 {
            return;
        }
        let (a, b) = self.row2_mut(i1, i2);
        a.swap_with_slice(b);
    }

    fn mat_add<A: Matrix<T> + ?Sized, B: Matrix<T> + ?Sized>(&mut self, a: &A, b: &B) {
        assert_eq!(self.size(), a.size());
        assert_eq!(self.size(), b.size());
        for i in 0..self.n() {
            for j in 0..self.m() {
                self.set_elem(i, j, a.elem(i, j) + b.elem(i, j));
            }
        }
    }

    fn mat_vec_mul<V: Vector<T> + ?Sized, U: Vector<T> + ?Sized>(&self, v: &V, dst: &mut U) {
        assert_eq!(self.m(), v.size());
        assert_eq!(self.n(), dst.size());
        for i in 0..self.n() {
            dst.set_elem(i, self.row(i).dot(v));
        }
    }

    fn mat_mul<A: Matrix<T> + ?Sized, B: Matrix<T> + ?Sized>(&mut self, a: &A, b: &B) {
        assert_eq!(a.m(), b.n());
        assert_eq!(a.n(), self.n());
        assert_eq!(b.m(), self.m());
        for i in 0..a.n() {
            for j in 0..b.m() {
                let mut dot = T::ZERO;
                for k in 0..a.m() {
                    dot = dot + a.elem(i, k) * b.elem(k, j);
                }
                self.set_elem(i, j, dot);
            }
        }
    }

    fn set_identity(&mut self) {
        assert_eq!(self.n(), self.m());
        for i in 0..self.n() {
            for j in 0..self.n() {
                self.set_elem(i, i, if i == j { T::ONE } else { T::ZERO });
            }
        }
    }

    fn transpose<M: Matrix<T> + ?Sized>(&self, dst: &mut M) {
        assert_eq!(self.n(), dst.m());
        assert_eq!(self.m(), dst.n());
        for i in 0..self.n() {
            for j in 0..self.m() {
                dst.set_elem(j, i, self.elem(i, j));
            }
        }
    }

    fn elimination(&mut self) {
        let mut p = 0;
        for k in 0..self.n() {
            while p < self.m() {
                let mut found = false;
                for i in k..self.n() {
                    if self.elem(i, p) != T::ZERO {
                        found = true;
                        self.swap_row(k, i);
                    }
                }
                if found {
                    let d = T::ONE / self.elem(k, p);
                    for i in k + 1..self.n() {
                        let a = self.elem(i, p) * d;
                        self.set_elem(i, p, T::ZERO);
                        for j in p + 1..self.m() {
                            let ek = self.elem(k, j);
                            let ei = self.elem_mut(i, j);
                            *ei = *ei - a * ek;
                        }
                    }
                    p += 1;
                    break;
                } else {
                    p += 1;
                }
            }
        }
    }
}

impl<T: Scalar, const N: usize, const M: usize> Matrix<T> for [[T; M]; N] {
    fn n(&self) -> usize {
        N
    }
    fn m(&self) -> usize {
        M
    }
    fn row(&self, i: usize) -> &[T] {
        &self[i]
    }
    fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self[i]
    }
    fn row2_mut(&mut self, i1: usize, i2: usize) -> (&mut [T], &mut [T]) {
        assert_ne!(i1, i2);
        #[allow(clippy::deref_addrof)]
        unsafe {
            (&mut *&raw mut self[i1], &mut *&raw mut self[i2])
        }
    }
}

pub struct MatBuf<T> {
    ptr: *mut T,
    _marker: PhantomData<T>,
}

#[repr(C)]
struct MatBufHeader<T> {
    n: usize,
    m: usize,
    _dummy: [T; 0],
}

impl<T> MatBuf<T> {
    fn header_ptr(&self) -> *const MatBufHeader<T> {
        unsafe { self.ptr.cast::<MatBufHeader<T>>().sub(1) }
    }
    fn header_mut_ptr(&mut self) -> *mut MatBufHeader<T> {
        unsafe { self.ptr.cast::<MatBufHeader<T>>().sub(1) }
    }
    fn header(&self) -> &MatBufHeader<T> {
        unsafe { &*self.header_ptr() }
    }
    pub fn as_flattened(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.header().n * self.header().m) }
    }
    pub fn as_flattened_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.header().n * self.header().m) }
    }
}

fn layout<T>(n: usize, m: usize) -> (Layout, usize) {
    Layout::new::<MatBufHeader<T>>()
        .extend(Layout::array::<T>(n * m).unwrap())
        .unwrap()
}

impl<T: Scalar> MatBuf<T> {
    pub fn zeros(n: usize, m: usize) -> Self {
        let (layout, offset) = layout::<T>(n, m);
        let ptr = unsafe { alloc::alloc(layout) };
        let header_ptr = ptr.cast::<MatBufHeader<T>>();
        let data_ptr = unsafe { ptr.add(offset).cast::<T>() };
        unsafe {
            header_ptr.write(MatBufHeader::<T> { n, m, _dummy: [] });
            for i in 0..n * m {
                data_ptr.add(i).write(T::ZERO);
            }
        }
        Self {
            ptr: data_ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for MatBuf<T> {
    fn drop(&mut self) {
        let &MatBufHeader { n, m, .. } = self.header();
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr, n * m));
        }
        unsafe {
            alloc::dealloc(self.header_mut_ptr().cast(), layout::<T>(n, m).0);
        }
    }
}

impl<T> Index<usize> for MatBuf<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        let m = self.header().m;
        &self.as_flattened()[i * m..(i + 1) * m]
    }
}

impl<T> IndexMut<usize> for MatBuf<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let m = self.header().m;
        &mut self.as_flattened_mut()[i * m..(i + 1) * m]
    }
}

impl<T: Scalar> Matrix<T> for MatBuf<T> {
    fn n(&self) -> usize {
        self.header().n
    }
    fn m(&self) -> usize {
        self.header().m
    }
    fn row(&self, i: usize) -> &[T] {
        &self[i]
    }
    fn row_mut(&mut self, i: usize) -> &mut [T] {
        let range = i * self.m()..(i + 1) * self.m();
        &mut self.as_flattened_mut()[range]
    }
    fn row2_mut(&mut self, i1: usize, i2: usize) -> (&mut [T], &mut [T]) {
        assert_ne!(i1, i2);
        unsafe {
            (
                slice::from_raw_parts_mut(self.ptr.add(self.m() * i1), self.m()),
                slice::from_raw_parts_mut(self.ptr.add(self.m() * i2), self.m()),
            )
        }
    }
}
