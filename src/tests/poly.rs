use crate::{modint::*, poly::*, simple_rng::*};

#[test]
fn dft_mul() {
    const M: u32 = 998244353;
    let mut a = Poly(vec![mint::<M>(1), mint(2)]);
    let mut b = Poly(vec![mint::<M>(3), mint(4), mint(5)]);
    a.dft_mul(&mut b);
    assert_eq!(a.as_slice(), [mint(3), mint(10), mint(13), mint(10)])
}

#[test]
fn inv_random() {
    const M: u32 = 998244353;
    let mut rng = Rng::new(216);
    for n in 1..=20 {
        for m in 1..=20 {
            let f = Poly((0..n).map(|_| ModInt::<M>(rng.range(..M))).collect());
            let g = f.inv(m);
            let mut prod = f.clone() * g;
            prod.truncate(m);
            prod.normalize();
            assert_eq!(prod.as_slice(), &[ModInt(1)], "n={n} m={m} f={f:?}");
        }
    }
}
