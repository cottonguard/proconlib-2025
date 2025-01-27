use crate::dsu::*;

#[test]
fn dsu_merge() {
    let n = 6;
    let mut dsu = DsuMerge::new_with(
        n,
        |i| vec![i],
        |a, b| {
            a.extend(b);
            a.sort();
        },
    );

    let (res, data) = dsu.unite(1, 2);
    assert!(res.is_united);
    assert_eq!(res.size, 2);
    assert_eq!(data, &[1, 2]);
    let r = res.root;

    let (res, data) = dsu.unite(2, 3);
    assert!(res.is_united);
    assert_eq!(res.size, 3);
    assert_eq!(res.root, r);
    assert_eq!(res.united_root, 3);
    assert_eq!(data, &[1, 2, 3]);

    let (res, data) = dsu.unite(1, 3);
    assert!(!res.is_united);
    assert_eq!(data, &[1, 2, 3]);

    dsu.unite(4, 5);

    let mut comps = dsu.comps();
    let (res, data) = comps.next().unwrap();
    assert_eq!(res.size, 1);
    assert_eq!(data, &[0]);
    let (res, data) = comps.next().unwrap();
    assert_eq!(res.size, 3);
    assert_eq!(data, &[1, 2, 3]);
    let (res, data) = comps.next().unwrap();
    assert_eq!(res.size, 2);
    assert_eq!(data, &[4, 5]);
    assert!(comps.next().is_none());

    let cloned = dsu.clone();
    assert!(dsu.comps().eq(cloned.comps()));

    let (res, data) = dsu.unite(3, 4);
    assert!(res.is_united);
    assert_eq!(res.size, 5);
    assert_eq!(res.root, r);
    assert_eq!(data, &[1, 2, 3, 4, 5]);
}

#[test]
fn dsu_merge_drop() {
    use std::rc::Rc;

    let rc = Rc::new(());

    let n = 10;
    let mut dsu = DsuMerge::new(n, rc.clone(), |_, _| {});
    assert_eq!(Rc::strong_count(&rc), n + 1);

    dsu.unite(0, 1);
    dsu.unite(2, 3);
    dsu.unite(1, 3);
    dsu.unite(4, 5);
    dsu.unite(6, 7);
    drop(dsu);

    assert_eq!(Rc::strong_count(&rc), 1);
}
