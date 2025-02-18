use crate::two_sat::*;

#[test]
fn test() {
    let mut ts = TwoSat::new(1);
    ts.clause(0, true, 0, true);
    assert_eq!(ts.solve(), Some(vec![true]));
}
