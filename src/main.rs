struct Solution {}

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();

        let mut left = 0usize;
        let mut right = 0usize;
        let mut chars = s.chars();
        let mut current = chars.next().unwrap();
        for next in chars {
            if current == next {
                right += 1;
                continue;
            }

            let len = right - left;
            if len >= 2 {
                result.push(vec![left as i32, right as i32]);
            }

            right += 1;
            left = right;
            current = next;
        }

        let len = right - left;
        if len >= 2 {
            result.push(vec![left as i32, right as i32]);
        }

        return result;
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input { s: "abbxxxxzzy" },
        Input { s: "abc" },
        Input {
            s: "abcdddeeeeaabbbcd",
        },
    ];

    for input in inputs {
        let result = Solution::large_group_positions(input.s.to_string());
        println!("{result:?}");
    }
}
