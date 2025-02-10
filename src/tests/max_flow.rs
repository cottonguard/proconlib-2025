use crate::max_frow::*;

#[test]
fn test() {
    let mut mf = MaxFrow::new(4);
    mf.edge(0, 1, 10);
    mf.edge(0, 2, 5);
    mf.edge(1, 2, 15);
    mf.edge(1, 3, 5);
    mf.edge(2, 3, 10);
    assert_eq!(mf.flow(0, 3), 15);

    let mut mf = MaxFrow::new(6);
    mf.edge(0, 1, 10);
    mf.edge(0, 2, 10);
    mf.edge(1, 2, 2);
    mf.edge(1, 3, 4);
    mf.edge(1, 4, 8);
    mf.edge(2, 4, 9);
    mf.edge(4, 3, 6);
    mf.edge(3, 5, 10);
    mf.edge(4, 5, 10);
    assert_eq!(mf.flow(0, 5), 19);
}
