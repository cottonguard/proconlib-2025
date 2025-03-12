use crate::matrix::*;

#[test]
fn vector() {
    let mut dst = [0; 3];
    dst.vec_add(&[1, 2, 3], &[10, 20, 30]);
    assert_eq!(dst, [11, 22, 33]);
    let mut v = [1, 2];
    v.scale(10);
    assert_eq!(v, [10, 20]);
    assert_eq!([1, 2].dot(&[3, 4]), 11);
}

#[test]
fn matbuf() {
    let _a = MatBuf::<f32>::zeros(3, 2);
}
