struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_occurrence = vec![0; 26];
        let mut n = 0usize;
        for (i, ch) in s.chars().enumerate() {
            let code = ch as u8;
            let a = b'a';
            let j = (code - a) as usize;
            last_occurrence[j] = i;
            n += 1;
        }

        let n = n;
        let last_occurrence = last_occurrence;
        let mut partition_start = 0usize;
        let mut partition_end = 0usize;
        let mut result = Vec::<i32>::with_capacity(n);
        for (i, ch) in s.chars().enumerate() {
            let code = ch as u8;
            let a = b'a';
            let j = (code - a) as usize;
            partition_end = partition_end.max(last_occurrence[j]);
            if i == partition_end {
                result.push((i + 1 - partition_start) as i32);
                partition_start = i + 1;
            }
        }

        return result;
    }
}

struct Input {
    s: &'static str,
}

fn main() {
    let inputs = vec![
        Input {
            s: "ababcbacadefegdehijhklij",
        },
        Input { s: "eccbbbbdec" },
    ];

    for input in inputs {
        let result = Solution::partition_labels(input.s.to_string());
        println!("{result:?}");
    }
}
