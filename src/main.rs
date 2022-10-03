struct Solution {}
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut cur = needed_time[0];

        let colors = colors.as_bytes();
        for i in 1..colors.len() {
            if colors[i] != colors[i - 1] {
                cur = 0;
            }

            let needed = needed_time[i];
            total += cur.min(needed);
            cur = cur.max(needed);
        }

        return total;
    }
}

struct Input {
    colors: &'static str,
    needed_time: Vec<i32>,
}
fn main() {
    let inputs = vec![
        Input {
            colors: "abaac",
            needed_time: vec![1, 2, 3, 4, 5],
        },
        Input {
            colors: "abc",
            needed_time: vec![1, 2, 3],
        },
        Input {
            colors: "aabaa",
            needed_time: vec![1, 2, 3, 4, 1],
        },
    ];

    for Input {
        colors,
        needed_time,
    } in inputs.into_iter()
    {
        let result = Solution::min_cost(colors.to_string(), needed_time);
        println!("{result:?}");
    }
}
