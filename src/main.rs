struct Solution {}
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        if (changed.len() & 1) == 1 {
            return vec![];
        }

        let mut changed = changed;
        changed.sort_unstable();

        let mut queue = Vec::<i32>::with_capacity(changed.len());
        let mut result = Vec::<i32>::with_capacity(changed.len() / 2);

        let mut head = 0usize;
        for i in changed.into_iter() {
            if let Some(n) = queue.get(head) {
                if *n == i {
                    head += 1;
                } else {
                    result.push(i);
                    queue.push(i * 2);
                }
            } else {
                result.push(i);
                queue.push(i * 2);
            }
        }

        if result.len() == head {
            result
        } else {
            vec![]
        }
    }
}

struct Input {
    changed: Vec<i32>,
}

fn main() {
    let inputs: Vec<Input> = vec![
        Input {
            changed: vec![1, 3, 4, 2, 6, 8],
        },
        Input {
            changed: vec![6, 3, 0, 1],
        },
        Input { changed: vec![1] },
        Input {
            changed: vec![0, 0, 0, 0],
        },
        Input {
            changed: vec![6, 3, 0, 1],
        },
        Input {
            changed: vec![2, 1],
        },
    ];

    for Input { changed } in inputs.into_iter() {
        let result = Solution::find_original_array(changed);
        println!("{result:?}");
    }
}
