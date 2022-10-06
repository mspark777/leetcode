use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(String, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        return TimeMap {
            store: HashMap::new(),
        };
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(nodes) = self.store.get_mut(&key) {
            nodes.push((value, timestamp));
        } else {
            self.store.insert(key, vec![(value, timestamp)]);
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let optnodes = self.store.get(&key);
        if optnodes.is_none() {
            return String::new();
        }

        let nodes = optnodes.unwrap();
        if timestamp < nodes[0].1 {
            return String::new();
        }

        let mut left = 0usize;
        let mut right = nodes.len();

        while left < right {
            let mid = (left + right) / 2;
            if nodes[mid].1 <= timestamp {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if right < 1 {
            return String::new();
        }

        return nodes[right - 1].0.clone();
    }
}

fn main() {
    let mut timemap = TimeMap::new();
    timemap.set("foo".to_string(), "bar".to_string(), 1);
    println!("{}", timemap.get("foo".to_string(), 1));
    println!("{}", timemap.get("foo".to_string(), 3));
    timemap.set("foo".to_string(), "bar2".to_string(), 4);
    println!("{}", timemap.get("foo".to_string(), 4));
    println!("{}", timemap.get("foo".to_string(), 5));
}
