struct Solution;

impl Solution {
    pub fn prison_after_n_days(mut cells: Vec<i32>, mut n: i32) -> Vec<i32> {
        n = match n % 14 == 0 {
            true => 14,
            false => n % 14,
        };
        let mut temp = cells.clone();
        temp[0] = 0;
        temp[7] = 0;

        for i in 1..7 {
            temp[i] = match cells[i - 1] == cells[i + 1] {
                true => 1,
                false => 0,
            };
        }

        n -= 1;
        cells = temp.clone();

        while n > 0 {
            for i in 1..7 {
                temp[i] = match cells[i - 1] == cells[i + 1] {
                    true => 1,
                    false => 0,
                };
            }

            n -= 1;
            cells = temp.clone();
        }

        cells
    }
}

struct Input {
    arr: Vec<i32>,
}

fn main() {
    let inputs = [Input {
        arr: [1, 2, 3, 4].to_vec(),
    }];

    for input in inputs {
        let result = Solution::can_reorder_doubled(input.arr);
        println!("{:?}", result);
    }
}
