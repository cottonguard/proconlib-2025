use crate::trie::*;

#[test]
fn trie() {
    let mut trie = Trie::new();
    assert_eq!(trie.count_node(), 1);

    assert_eq!(trie.insert_slice(b"abc"), (true, 3));
    assert_eq!(trie.insert_slice(b"axz"), (true, 5));
    assert_eq!(trie.insert_slice(b"aba"), (true, 6));
    assert_eq!(trie.insert_slice(b"aba"), (false, 6));

    let mut links = trie.links(1);
    assert_eq!(links.next(), Some((b'b', 2)));
    assert_eq!(links.next(), Some((b'x', 4)));
    assert_eq!(links.next(), None);
}
