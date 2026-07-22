#[derive(Default)]
struct Node {
    sum: i32,
    val: i32,
    children: [Option<Box<Node>>; 26],
}

struct MapSum {
    root: Node,
}

impl MapSum {
    fn new() -> Self {
        let root = Node::default();
        Self { root }
    }

    fn insert(&mut self, key: String, val: i32) {
        let mut old_val = 0;
        let mut n = &self.root;
        for c in key.bytes() {
            let b = (c - b'a') as usize;
            match &n.children[b] {
                Some(node) => {
                    n = node;
                    old_val = n.val;
                }
                _ => {
                    old_val = 0;
                    break;
                }
            }
        }

        let mut n = &mut self.root;
        n.sum += val - old_val;
        for b in key.bytes() {
            let c = (b - b'a') as usize;
            match &mut n.children[c] {
                m @ None => {
                    *m = Some(Box::new(Node::default()));
                    n = m.as_mut().unwrap();
                }
                Some(node) => n = node,
            }
            n.sum += val - old_val;
        }
        n.val = val;
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut n = &self.root;
        for c in prefix.bytes() {
            let b = (c - b'a') as usize;
            match &n.children[b] {
                Some(node) => n = &node,
                _ => return 0,
            }
        }

        n.sum
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */

struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lengths = vec![1; n];
        let mut counts = vec![1; n];

        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if (lengths[j] + 1) > lengths[i] {
                        lengths[i] = lengths[j] + 1;
                        counts[i] = 0;
                    }
                    if (lengths[j] + 1) == lengths[i] {
                        counts[i] += counts[j];
                    }
                }
            }
        }

        let max_length = lengths.iter().copied().max().unwrap();
        let mut result = 0;
        for (i, l) in lengths.into_iter().enumerate() {
            if l == max_length {
                result += counts[i];
            }
        }

        result
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        nums: [1, 3, 5, 4, 7].to_vec(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::find_number_of_lis(input.nums);
        println!("{:?}", result);
    }
}
