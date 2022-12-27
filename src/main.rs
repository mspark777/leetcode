struct Solution {}
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut remains: Vec<i32> = capacity
            .iter()
            .enumerate()
            .map(|(i, c)| c - rocks[i])
            .collect();
        remains.sort_unstable();

        let mut additional = additional_rocks;
        let mut result = 0;
        for &remain in remains.iter() {
            if additional >= remain {
                additional -= remain;
                result += 1;
            } else {
                break;
            }
        }

        return result;
    }
}

struct Input {
    capacity: Vec<i32>,
    rocks: Vec<i32>,
    additional_rocks: i32,
}
fn main() {
    let inputs = [
        Input {
            capacity: vec![2, 3, 4, 5],
            rocks: vec![1, 2, 4, 4],
            additional_rocks: 2,
        },
        Input {
            capacity: vec![10, 2, 2],
            rocks: vec![2, 2, 0],
            additional_rocks: 100,
        },
    ];

    for Input {
        capacity,
        rocks,
        additional_rocks,
    } in inputs
    {
        let result = Solution::maximum_bags(capacity, rocks, additional_rocks);
        println!("{result}");
    }
}
