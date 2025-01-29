use std::collections::VecDeque;

use crate::trie::*;

pub struct AhoCorasick {
    trie: Trie,
    suf_link: Vec<usize>,
    depth: Vec<usize>,
}

impl AhoCorasick {
    pub fn from_trie(trie: Trie) -> Self {
        let mut suf_link = vec![0; trie.count_node()];
        let mut depth = vec![0; trie.count_node()];
        let mut que: VecDeque<usize> = vec![0].into();
        while let Some(u) = que.pop_front() {
            for (c, v) in trie.links(u) {
                if u != 0 {
                    let mut a = u;
                    loop {
                        if let Some(w) = trie.transition(suf_link[a], c) {
                            suf_link[v] = w;
                            break;
                        }
                        if a == 0 {
                            break;
                        }
                        a = suf_link[a];
                    }
                }
                depth[v] = depth[u] + 1;
                que.push_back(v);
            }
        }

        Self {
            trie,
            suf_link,
            depth,
        }
    }

    pub fn transition(&self, mut i: usize, c: u8) -> Option<usize> {
        loop {
            if let Some(j) = self.trie.transition(i, c) {
                return Some(j);
            }
            let Some(i_) = self.suffix(i) else {
                return None;
            };
            i = i_;
        }
    }

    pub fn depth(&self, i: usize) -> usize {
        self.depth[i]
    }

    pub fn suffix(&self, i: usize) -> Option<usize> {
        (i != 0).then(|| self.suf_link[i])
    }
}
