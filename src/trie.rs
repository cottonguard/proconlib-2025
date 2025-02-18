#[derive(Clone)]
pub struct Trie {
    nodes: Vec<Node>,
}

type Bits = u64;

#[derive(Clone)]
struct Node {
    bits: [Bits; 4],
    edges: Vec<usize>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
        }
    }

    pub fn count_node(&self) -> usize {
        self.nodes.len()
    }

    pub fn transition(&self, i: usize, c: u8) -> Option<usize> {
        self.nodes[i].transition(c)
    }

    pub fn links(&self, i: usize) -> Links {
        Links {
            node: &self.nodes[i],
            c: Some(0),
        }
    }

    pub fn insert_slice(&mut self, s: &[u8]) -> (bool, usize) {
        self.insert(s.iter().copied())
    }

    pub fn insert(&mut self, s: impl IntoIterator<Item = u8>) -> (bool, usize) {
        let mut i = 0;
        let mut inserted = false;
        for c in s {
            let len = self.nodes.len();
            if let Some(node) = self.nodes.get_mut(i) {
                if node.edges.is_empty() {
                    inserted = true;
                    node.set_bit(c);
                    node.edges.push(len);
                    i = len;
                    self.nodes.push(Node::new());
                } else {
                    let (exists, rank) = node.rank(c);
                    if !exists {
                        inserted = true;
                        node.set_bit(c);
                        node.edges.insert(rank, len);
                        i = len;
                        self.nodes.push(Node::new());
                    } else {
                        i = node.edges[rank];
                    }
                }
            }
        }
        (inserted, i)
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    fn new() -> Self {
        Self {
            bits: [0; 4],
            edges: vec![],
        }
    }

    #[inline]
    fn transition(&self, c: u8) -> Option<usize> {
        let (exists, i) = self.rank(c);
        exists.then(|| self.edges[i])
    }

    #[inline]
    fn set_bit(&mut self, c: u8) {
        let (q, r) = index_bit(c);
        self.bits[q] |= 1 << r;
    }

    #[inline]
    fn rank(&self, c: u8) -> (bool, usize) {
        let (q, r) = index_bit(c);
        let rank: u32 = (0..=q)
            .map(|i| {
                let bits = if i < q {
                    self.bits[i]
                } else {
                    self.bits[i] & (1 << r) - 1
                };
                bits.count_ones()
            })
            .sum();
        let set = self.bits[q] & (1 << r) != 0;
        (set, rank as usize)
    }
}

fn index_bit(c: u8) -> (usize, usize) {
    (
        c as usize / Bits::BITS as usize,
        c as usize % Bits::BITS as usize,
    )
}

pub struct Links<'a> {
    node: &'a Node,
    c: Option<u8>,
}

impl Iterator for Links<'_> {
    type Item = (u8, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.c?;
        let (mut q, mut r) = index_bit(c);
        while q < 4 {
            let bits = self.node.bits[q] & !((1 << r) - 1);
            if bits != 0 {
                let r = bits.trailing_zeros() as usize;
                let c = (Bits::BITS as usize * q + r) as u8;
                self.c = c.checked_add(1);
                return Some((c, self.node.transition(c).unwrap()));
            }
            q += 1;
            r = 0;
        }
        None
    }
}
