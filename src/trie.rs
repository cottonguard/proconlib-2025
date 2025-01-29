pub struct Trie {
    nodes: Vec<Node>,
}

type Bits = u64;

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

    pub fn transition(&self, i: usize, c: u8) -> Option<usize> {
        self.nodes[i].transition(c)
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
                } else {
                    let (exists, rank) = node.rank(c);
                    if !exists {
                        inserted = true;
                        node.set_bit(c);
                        node.edges.insert(rank, len);
                        i = len;
                        self.nodes.push(Node::new());
                    } else {
                        i = rank;
                    }
                }
            }
        }
        (inserted, i)
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
