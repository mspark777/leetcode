struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        let mut first = 0usize;
        let mut second = 0usize;

        while (first < first_list.len()) && (second < second_list.len()) {
            let begin = first_list[first][0].max(second_list[second][0]);
            let end = first_list[first][1].min(second_list[second][1]);

            if begin <= end {
                ans.push(vec![begin, end]);
            }

            if second_list[second][1] > first_list[first][1] {
                first += 1;
            } else {
                second += 1;
            }
        }
        ans
    }
}

struct Input {
    x: i32,
    y: i32,
    bound: i32,
}

fn main() {
    let inputs = [Input {
        x: 2,
        y: 3,
        bound: 10,
    }];

    for input in inputs {
        let result = Solution::powerful_integers(input.x, input.y, input.bound);
        println!("{:?}", result);
    }
}
