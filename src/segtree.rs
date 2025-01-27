use std::ops::{Bound, Deref, RangeBounds};

pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}

pub struct SegTree<T> {
    a: Vec<T>,
}

impl<T: Monoid> SegTree<T> {
    pub fn new(n: usize) -> Self {
        Self {
            a: (0..2 * n).map(|_| T::id()).collect(),
        }
    }

    pub fn set(&mut self, i: usize, x: T) {
        assert!(
            i < self.len(),
            "out of range (len = {}, index = {i})",
            self.len()
        );
        let mut i = self.node_index(i);
        self.a[i] = x;
        loop {
            i /= 2;
            if i == 0 {
                break;
            }
            self.a[i] = self.a[2 * i].op(&self.a[2 * i + 1]);
        }
    }

    pub fn prod(&self, range: impl RangeBounds<usize>) -> T {
        if range.start_bound() == Bound::Unbounded && range.end_bound() == Bound::Unbounded {
            return self.a[1].op(&T::id());
        }

        let l = match range.start_bound() {
            Bound::Included(&l) => l,
            Bound::Excluded(&l) => l + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&r) => r + 1,
            Bound::Excluded(&r) => r,
            Bound::Unbounded => self.len(),
        };

        self.prod_impl(l, r)
    }

    fn prod_impl(&self, l: usize, r: usize) -> T {
        assert!(r <= self.len());
        assert!(l <= r);

        if l == r {
            return T::id();
        }

        let mut l = self.node_index(l);
        l >>= l.trailing_zeros();
        let mut r = self.node_index(r);
        r >>= r.trailing_zeros();
        let mut x = T::id();
        let mut y = T::id();
        loop {
            if l >= r {
                x = x.op(&self.a[l]);
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
                y = self.a[r].op(&y);
                r >>= r.trailing_zeros();
            }
            if l == r {
                break;
            }
        }
        x.op(&y)
    }

    #[inline]
    fn node_index(&self, i: usize) -> usize {
        let i = i + self.len().next_power_of_two();
        if i >= self.a.len() { i - self.len() } else { i }
    }

    pub fn max_right(&self, l: usize, mut f: impl FnMut(&T) -> bool) -> (usize, T) {
        assert!(l <= self.len());
        let mut prod = T::id();
        if l == self.len() {
            return (l, prod);
        }
        let mut r = self.node_index(l);
        loop {
            r >>= r.trailing_zeros();
            loop {
                let prod_new = prod.op(&self.a[r]);
                if f(&prod_new) {
                    prod = prod_new;
                    r += 1;
                    if r.count_ones() <= 1 {
                        return (self.len(), prod);
                    }
                    break;
                }
                if r >= self.len() {
                    let r = if r >= self.len().next_power_of_two() {
                        r - self.len().next_power_of_two()
                    } else {
                        r + self.len() - self.len().next_power_of_two()
                    };
                    return (r, prod);
                }
                r *= 2;
            }
        }
    }

    pub fn min_left(&self, r: usize, mut f: impl FnMut(&T) -> bool) -> (usize, T) {
        assert!(r <= self.len());
        let mut prod = T::id();
        if r == 0 {
            return (0, prod);
        }
        let mut l = self.node_index(r);
        loop {
            l = (l >> l.trailing_zeros()).max(2);
            loop {
                let prod_new = self.a[l - 1].op(&prod);
                if f(&prod_new) {
                    prod = prod_new;
                    l -= 1;
                    if l.count_ones() <= 1 {
                        return (0, prod);
                    }
                    break;
                }
                if l > self.len() {
                    let l = if l > self.len().next_power_of_two() {
                        l - self.len().next_power_of_two()
                    } else {
                        l + self.len() - self.len().next_power_of_two()
                    };
                    return (l, prod);
                }
                l *= 2;
            }
        }
    }
}

impl<T> Deref for SegTree<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.a[self.a.len() / 2..]
    }
}

impl<T: Monoid> From<Vec<T>> for SegTree<T> {
    fn from(mut a: Vec<T>) -> Self {
        if let Some(add) = (2 * a.len()).checked_sub(a.capacity()) {
            a.reserve(add);
        }
        let n = a.len();
        unsafe {
            a.set_len(0);
            let ptr = a.as_mut_ptr();
            ptr.copy_to(ptr.add(n), n);
            let r = n.next_power_of_two() - n;
            let l = n - r;

            ptr.add(l)
                .copy_to_nonoverlapping(ptr.add(n.next_power_of_two() - r), r);
            ptr.copy_to_nonoverlapping(ptr.add(n.next_power_of_two()), l);
            for i in (1..n).rev() {
                ptr.add(i)
                    .write((&*ptr.add(2 * i)).op(&*ptr.add(2 * i + 1)));
            }
            ptr.write(T::id());
            a.set_len(2 * n);
        }
        Self { a }
    }
}
