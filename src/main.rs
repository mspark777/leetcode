struct Solution;

impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut lights = vec![false; 100];

        for bulb in bulbs {
            let idx = (bulb - 1) as usize;
            lights[idx] = !lights[idx];
        }

        lights
            .into_iter()
            .enumerate()
            .filter(|&(_, l)| l)
            .map(|(i, _)| (i + 1) as i32)
            .collect()
    }
}

struct Input {
    bulbs: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            bulbs: [10, 30, 20, 10].to_vec(),
        },
        Input {
            bulbs: [100, 100].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::toggle_light_bulbs(input.bulbs);
        println!("{:?}", result);
    }
}
