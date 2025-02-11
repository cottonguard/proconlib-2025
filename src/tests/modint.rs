use crate::modint::*;

#[test]
fn test() {
    const M: u32 = 998244353;
    let x = mint::<M>(123);
    assert_eq!(x * x.inv(), mint(1));
}
