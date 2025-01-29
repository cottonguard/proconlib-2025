use crate::{aho_corasick::*, trie::*};

#[test]
fn test() {
    let mut trie = Trie::new();
    trie.insert_slice(b"abc");
    trie.insert_slice(b"ast");
    trie.insert_slice(b"bcd");

    let ac = AhoCorasick::from_trie(trie);
    assert_eq!(ac.transition(0, b'a'), Some(1));
    assert_eq!(ac.transition(1, b'b'), Some(2));
    assert_eq!(ac.suffix(2), Some(6));
}
