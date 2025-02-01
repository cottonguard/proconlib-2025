use crate::{lazy_segtree::*, simple_rng::Rng};

#[test]
fn range_affine_range_sum() {
    let mut rng = Rng::new(20250124);
    for n in 1..=50 {
        for _ in 0..20 {
            let mut a: Vec<u32> = (0..n).map(|_| rng.next_u32() % 64).collect();
            let m: Vec<M> = a.iter().map(|a| M(*a, 1)).collect();
            let mut st: LazySegTree<M, A> = m.into();
            for it in 0..20 {
                if it % 5 == 0 {
                    let i = (rng.next_u32() as usize) % n;
                    let x = rng.next_u32() % 64;
                    assert_eq!(st.set(i, M(x, 1)).0, a[i]);
                    a[i] = x;
                } else {
                    let l = (rng.next_u32() as usize) % (n + 1);
                    let r = (rng.next_u32() as usize) % (n + 1);
                    let (l, r) = if l <= r { (l, r) } else { (r, l) };
                    let f = A(rng.next_u32() % 8, rng.next_u32() % 64);
                    for i in l..r {
                        a[i] = f.map(&M(a[i], 1)).0;
                    }
                    st.apply(l..r, f);
                }

                let l = (rng.next_u32() as usize) % (n + 1);
                let r = (rng.next_u32() as usize) % (n + 1);
                let (l, r) = if l <= r { (l, r) } else { (r, l) };

                // eprintln!("a = {a:?}");
                // eprintln!("st = {st:?}");

                let prod_a: u32 = a[l..r].iter().fold(0, |acc, v| acc.wrapping_add(*v));
                let prod_st = st.prod(l..r);

                assert_eq!(prod_a, prod_st.0, "l={l}, r={r}, a={a:?}");
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct M(u32, u32);

impl Monoid for M {
    fn id() -> Self {
        M(0, 0)
    }
    fn op(&self, other: &Self) -> Self {
        M(self.0.wrapping_add(other.0), self.1.wrapping_add(other.1))
    }
}

impl Map<M> for A {
    fn id() -> Self {
        A(1, 0)
    }
    fn comp(&self, other: &Self) -> Self {
        let A(a, b) = *self;
        let A(c, d) = *other;
        // a(cx + d) + b = acx + ad + b
        A(a.wrapping_mul(c), a.wrapping_mul(d).wrapping_add(b))
    }
    fn map(&self, x: &M) -> M {
        let A(a, b) = *self;
        let M(x, c) = *x;
        M(a.wrapping_mul(x).wrapping_add(b.wrapping_mul(c)), c)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct A(u32, u32);

impl Monoid for A {
    fn id() -> Self {
        A(1, 0)
    }
    fn op(&self, other: &Self) -> Self {
        let A(a, b) = *self;
        let A(c, d) = *other;
        // a(cx + d) + b = acx + ad + b
        A(a.wrapping_mul(c), a.wrapping_mul(d).wrapping_add(b))
    }
}
