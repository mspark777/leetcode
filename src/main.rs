use std::collections::HashMap;
use std::collections::HashSet;

struct Twitter {
    tweets: Vec<(i32, i32)>,
    users: HashMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self {
            tweets: vec![],
            users: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let empty = HashSet::<i32>::new();
        let followees = match self.users.get(&user_id) {
            Some(list) => list,
            _ => &empty,
        };

        let mut list = Vec::<i32>::with_capacity(10);
        for (user, tweet) in self.tweets.iter().rev().copied() {
            if user == user_id {
                list.push(tweet);
            } else if followees.contains(&user) {
                list.push(tweet);
            }

            if list.len() >= 10 {
                break;
            }
        }

        list
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let followees = self
            .users
            .entry(follower_id)
            .or_insert_with(|| HashSet::new());
        followees.insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.users.get_mut(&follower_id) {
            followees.remove(&followee_id);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let amount = Self::travel(root);
        amount.0.max(amount.1)
    }

    fn travel(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            Some(n) => Self::travel1(n.clone()),
            _ => (0, 0),
        }
    }

    fn travel1(node: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let left = Self::travel(node.borrow().left.clone());
        let right = Self::travel(node.borrow().right.clone());

        (
            left.0.max(left.1) + right.0.max(right.1),
            node.borrow().val + left.0 + right.0,
        )
    }
}

struct Input {
    preorder: String,
}

fn main() {
    let inputs = [Input {
        preorder: "9,#,92,#,#".to_string(),
    }];

    for input in inputs.into_iter() {
        let result = Solution::is_valid_serialization(input.preorder);
        println!("{:?}", result);
    }
}
