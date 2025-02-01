use std::ops::{Bound, RangeBounds};

pub trait Monoid {
    fn id() -> Self;
    fn op(&self, other: &Self) -> Self;
}

pub trait Map<T> {
    fn id() -> Self;
    fn comp(&self, other: &Self) -> Self;
    fn map(&self, x: &T) -> T;
}

#[derive(Clone, Debug)]
pub struct LazySegTree<T, M> {
    value: Vec<T>,
    map: Vec<M>,
}

impl<T: Monoid, M: Map<T>> LazySegTree<T, M> {
    fn len(&self) -> usize {
        self.value.len() / 2
    }

    pub fn prod(&self, range: impl RangeBounds<usize>) -> T {
        if range.start_bound() == Bound::Unbounded && range.end_bound() == Bound::Unbounded {
            return self.value[1].op(&T::id());
        }

        let (l, r) = self.range(range);
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
                let mut p = l / 2;
                x = x.op(&self.value[l]);
                l += 1;
                l >>= l.trailing_zeros();
                while p > 0 && p + 1 >= l {
                    x = self.map[p].map(&x);
                    p /= 2;
                }
            } else {
                let mut p = r / 2;
                r -= 1;
                y = self.value[r].op(&y);
                r >>= r.trailing_zeros();
                while p >= r {
                    y = self.map[p].map(&y);
                    p /= 2;
                }
            }
            if l == r {
                break;
            }
        }
        x = x.op(&y);
        let mut p = l / 2;
        while p > 0 {
            x = self.map[p].map(&x);
            p /= 2;
        }
        x
    }

    pub fn apply(&mut self, range: impl RangeBounds<usize>, map: M) {
        let (l, r) = self.range(range);
        self.apply_impl(l, r, map);
    }

    fn apply_impl(&mut self, l: usize, r: usize, map: M) {
        if l == r {
            return;
        }

        let mut l = self.node_index(l);
        l >>= l.trailing_zeros();
        let mut r = self.node_index(r);
        r >>= r.trailing_zeros();

        let nl = l.ilog2();
        let nr = r.ilog2();
        for i in 0..nl.max(nr) {
            if nl > i {
                let al = l >> nl - i;
                self.propagate(al);
            }
            if nr > i {
                let ar = r >> nr - i;
                if !(nl > i && ar == l >> nl - i) {
                    self.propagate(ar);
                }
            }
        }

        loop {
            if l >= r {
                let mut p = l / 2;
                self.value[l] = map.map(&self.value[l]);
                if l < self.map.len() {
                    self.map[l] = map.comp(&self.map[l]);
                }
                l += 1;
                l >>= l.trailing_zeros();
                while p > 0 && p + 1 >= l {
                    self.value[p] = self.value[2 * p].op(&self.value[2 * p + 1]);
                    p /= 2;
                }
            } else {
                let mut p = r / 2;
                r -= 1;
                self.value[r] = map.map(&self.value[r]);
                if r < self.map.len() {
                    self.map[r] = map.comp(&self.map[r]);
                }
                r >>= r.trailing_zeros();
                while p >= r {
                    self.value[p] = self.value[2 * p].op(&self.value[2 * p + 1]);
                    p /= 2;
                }
            }
            if l == r {
                break;
            }
        }
        let mut p = l / 2;
        while p > 0 {
            self.value[p] = self.value[2 * p].op(&self.value[2 * p + 1]);
            p /= 2;
        }
    }

    #[inline]
    fn propagate(&mut self, i: usize) {
        self.value[2 * i] = self.map[i].map(&self.value[2 * i]);
        self.value[2 * i + 1] = self.map[i].map(&self.value[2 * i + 1]);
        if 2 * i < self.map.len() {
            self.map[2 * i] = self.map[i].comp(&self.map[2 * i]);
        }
        if 2 * i + 1 < self.map.len() {
            self.map[2 * i + 1] = self.map[i].comp(&self.map[2 * i + 1]);
        }
        self.map[i] = M::id();
    }

    #[inline]
    fn node_index(&self, i: usize) -> usize {
        let i = i + self.len().next_power_of_two();
        if i >= self.value.len() {
            i - self.len()
        } else {
            i
        }
    }

    #[inline]
    fn range(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
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
        (l, r)
    }
}

impl<T: Monoid, M: Map<T>> From<Vec<T>> for LazySegTree<T, M> {
    fn from(mut value: Vec<T>) -> Self {
        if let Some(add) = (2 * value.len()).checked_sub(value.capacity()) {
            value.reserve(add);
        }
        let n = value.len();
        unsafe {
            value.set_len(0);
            let ptr = value.as_mut_ptr();
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
            value.set_len(2 * n);
        }
        let map = (0..n).map(|_| M::id()).collect();
        Self { value, map }
    }
}
