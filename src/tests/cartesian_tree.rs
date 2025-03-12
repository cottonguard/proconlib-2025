use crate::cartesian_tree::*;

#[test]
fn cartesian_tree_test() {
    assert_eq!(cartesian_tree_up(&[1, 2, 3]), [!0, 0, 1]);
    assert_eq!(
        cartesian_tree_down(&[1, 2, 3]),
        [(!0, 1), (!0, 2), (!0, !0)]
    );
    assert_eq!(cartesian_tree_up(&[1, 1, 1]), [!0, 0, 1]);
    assert_eq!(cartesian_tree_up(&[3, 1, 2]), [1, !0, 1]);
    assert_eq!(
        cartesian_tree_down(&[3, 1, 2]),
        [(!0, !0), (0, 2), (!0, !0)]
    );
    assert_eq!(cartesian_tree_up(&[3, 2, 1]), [1, 2, !0]);
}
