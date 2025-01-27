use crate::{segtree::*, simple_rng::Rng};

#[test]
fn affine() {
    let mut rng = Rng::new(20250124);
    for n in 1..=50 {
        for _ in 0..20 {
            let mut a: Vec<A> = (0..n)
                .map(|_| A(rng.next_u32() % 16, rng.next_u32() % 16))
                .collect();
            let mut st: SegTree<A> = a.clone().into();
            for _ in 0..20 {
                let l = (rng.next_u32() as usize) % (n + 1);
                let r = (rng.next_u32() as usize) % (n + 1);
                let (l, r) = if l <= r { (l, r) } else { (r, l) };

                let prod_a: A = a[l..r].iter().fold(A::id(), |acc, v| acc.op(&v));
                let prod_st = st.prod(l..r);

                assert_eq!(prod_a, prod_st, "l={l}, r={r}, a={a:?}");

                let i = (rng.next_u32() as usize) % n;
                let v = A(rng.next_u32(), rng.next_u32());
                a[i] = v;
                st.set(i, v);
            }
        }
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

#[test]
fn bisect() {
    let mut rng = Rng::new(20250125);
    for n in 1..=50 {
        for _ in 0..20 {
            let mut a: Vec<u32> = (0..n).map(|_| rng.next_u32() % 256).collect();
            let mut st: SegTree<u32> = a.clone().into();
            for _ in 0..20 {
                let i = (rng.next_u32() as usize) % n;
                let x = rng.next_u32() % 256;
                a[i] = x;
                st.set(i, x);

                let l = (rng.next_u32() as usize) % n;
                let x = rng.next_u32() % (256 * (n - l).max(1) as u32);
                let mut r = l;
                let mut sum = 0;
                while r < n && sum + a[r] <= x {
                    sum += a[r];
                    r += 1;
                }
                assert_eq!(r, st.max_right(l, |&sum| sum <= x), "a={a:?}, x={x}");
            }
        }
    }
}

impl Monoid for u32 {
    fn id() -> Self {
        0
    }
    fn op(&self, other: &Self) -> Self {
        self + other
    }
}
