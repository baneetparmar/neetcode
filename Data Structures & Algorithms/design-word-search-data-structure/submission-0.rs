struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    is_end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            children: std::array::from_fn(|_| None),
            is_end: false,
        }
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;
        for c in word.bytes().map(|b| (b - b'a') as usize) {
            node = node.children[c].get_or_insert_with(|| Box::new(WordDictionary::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.search_bytes(word.as_bytes())
    }

    fn search_bytes(&self, word: &[u8]) -> bool {
        if word.is_empty() { return self.is_end; }
        let c = word[0];
        if c == b'.' {
            self.children.iter()
                .filter_map(|child| child.as_deref())
                .any(|child| child.search_bytes(&word[1..]))
        } else {
            let idx = (c - b'a') as usize;
            self.children[idx].as_deref()
                .map_or(false, |child| child.search_bytes(&word[1..]))
        }
    }
}