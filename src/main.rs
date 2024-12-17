struct Solution {}
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut frequency_map = std::collections::HashMap::<char, i32>::new();
        for ch in s.chars() {
            frequency_map
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut queue = std::collections::BinaryHeap::<char>::with_capacity(frequency_map.len());
        for &ch in frequency_map.keys() {
            queue.push(ch);
        }

        let mut result = Vec::<char>::new();
        while let Some(ch) = queue.pop() {
            let count = frequency_map[&ch];
            let repeat = count.min(repeat_limit);
            for _ in 0..repeat {
                result.push(ch);
            }

            frequency_map.entry(ch).and_modify(|count| *count -= repeat);
            if frequency_map[&ch] <= 0 {
                continue;
            } else if queue.is_empty() {
                continue;
            }

            let next_ch = queue.pop().unwrap();
            result.push(next_ch);

            frequency_map.entry(next_ch).and_modify(|count| *count -= 1);
            if frequency_map[&next_ch] > 0 {
                queue.push(next_ch);
            }

            queue.push(ch);
        }

        return result.iter().collect();
    }
}

struct Input {
    s: &'static str,
    repeat_limit: i32,
}

fn main() {
    let inputs = vec![
        Input {
            s: "cczazcc",
            repeat_limit: 3,
        },
        Input {
            s: "aababab",
            repeat_limit: 2,
        },
    ];

    for input in inputs {
        let result = Solution::repeat_limited_string(input.s.to_string(), input.repeat_limit);
        println!("{result:?}");
    }
}
