struct PrefixTree {
    children: [Option<Box<PrefixTree>>; 26],
    is_end: bool,
}

impl PrefixTree {
    fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.bytes().map(|b| (b - b'a') as usize) {
            node = node.children[c].get_or_insert_with(|| Box::new(PrefixTree::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.get_node(&word).map_or(false, |n| n.is_end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.get_node(&prefix).is_some()
    }

    fn get_node(&self, s: &str) -> Option<&PrefixTree> {
        let mut node = self;
        for c in s.bytes().map(|b| (b - b'a') as usize) {
            node = node.children[c].as_deref()?;
        }
        Some(node)
    }
}