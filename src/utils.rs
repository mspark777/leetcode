use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

#[allow(dead_code)]
#[derive(Clone)]
struct TrieNode {
    links: Vec<Option<Box<TrieNode>>>,
    ended: bool,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct Trie {
    root: Box<TrieNode>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct UnionFind {
    parents: Vec<i32>,
    ranks: Vec<i32>,
}

#[allow(dead_code)]
impl UnionFind {
    pub fn new(size: i32) -> Self {
        let parents: Vec<i32> = (0..size).collect();
        let ranks = vec![0; size as usize];

        return Self { parents, ranks };
    }

    pub fn find(&mut self, x: i32) -> i32 {
        let i = x as usize;
        if self.parents[i] != x {
            self.parents[i] = self.find(self.parents[i])
        }

        return self.parents[i];
    }

    pub fn union(&mut self, x: i32, y: i32) {
        let xset = self.find(x);
        let yset = self.find(y);
        if xset == yset {
            return;
        }

        let xi = xset as usize;
        let yi = yset as usize;
        if self.ranks[xi] < self.ranks[yi] {
            self.parents[xi] = yset;
        } else if self.ranks[xi] > self.ranks[yi] {
            self.parents[yi] = xset;
        } else {
            self.parents[yi] = xset;
            self.ranks[xi] += 1;
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    pub fn new_node(
        val: i32,
        left: Option<Rc<RefCell<Self>>>,
        right: Option<Rc<RefCell<Self>>>,
    ) -> Option<Rc<RefCell<Self>>> {
        return Some(Rc::new(RefCell::new(Self { val, left, right })));
    }

    pub fn new_left(val: i32, left: Option<Rc<RefCell<Self>>>) -> Option<Rc<RefCell<Self>>> {
        return Self::new_node(val, left, None);
    }

    pub fn new_right(val: i32, right: Option<Rc<RefCell<Self>>>) -> Option<Rc<RefCell<Self>>> {
        return Self::new_node(val, None, right);
    }

    pub fn new_val(val: i32) -> Option<Rc<RefCell<Self>>> {
        return Self::new_node(val, None, None);
    }
}
