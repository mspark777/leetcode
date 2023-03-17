#[derive(Clone)]
struct TrieNode {
    links: Vec<Option<Box<TrieNode>>>,
    ended: bool,
}

impl TrieNode {
    fn new() -> Box<Self> {
        return Box::new(Self {
            links: vec![None; 26],
            ended: false,
        });
    }

    fn get(&self, ch: char) -> Option<&Box<TrieNode>> {
        let i = Self::get_index(ch);
        if self.in_range(i) {
            let link = &self.links[i as usize];
            return link.as_ref();
        }

        return None;
    }

    fn get_mut(&mut self, ch: char) -> Option<&mut Box<TrieNode>> {
        let i = Self::get_index(ch);
        if self.in_range(i) {
            let link = &mut self.links[i as usize];
            return link.as_mut();
        }

        return None;
    }

    fn contains_key(&self, ch: char) -> bool {
        return self.get(ch).is_some();
    }

    fn put(&mut self, ch: char, node: Box<TrieNode>) {
        let i = Self::get_index(ch);
        if self.in_range(i) {
            self.links[i as usize] = Some(node);
        }
    }

    fn set_end(&mut self) {
        self.ended = true;
    }

    fn is_end(&self) -> bool {
        return self.ended;
    }

    fn get_index(ch: char) -> i32 {
        return (ch as i32) - ('a' as i32);
    }

    fn in_range(&self, i: i32) -> bool {
        return (i >= 0) && ((i as usize) < self.links.len());
    }
}

pub struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        return Self {
            root: TrieNode::new(),
        };
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if !node.contains_key(ch) {
                node.put(ch, TrieNode::new());
            }

            node = node.get_mut(ch).unwrap();
        }

        node.set_end();
    }

    pub fn search(&self, word: String) -> bool {
        if let Some(node) = self.search_prefix(&word) {
            return node.is_end();
        } else {
            return false;
        }
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        return self.search_prefix(&prefix).is_some();
    }

    fn search_prefix(&self, prefix: &String) -> Option<&Box<TrieNode>> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            if let Some(n) = node.get(ch) {
                node = n;
            } else {
                return None;
            }
        }

        return Some(node);
    }
}
