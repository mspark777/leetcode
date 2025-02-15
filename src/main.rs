struct Solution {}

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut result = 0;

        for num in 1..=n {
            let square = num * num;

            if Self::can_partition(square.to_string().chars().collect(), num) {
                result += square;
            }
        }

        return result;
    }

    fn can_partition(str_num: Vec<char>, target: i32) -> bool {
        if str_num.is_empty() && target == 0 {
            return true;
        } else if target < 0 {
            return false;
        }

        for i in 1..str_num.len() {
            let left = str_num[0..i].to_vec();
            let right = str_num[i..].to_vec();
            let left_num = left.iter().collect::<String>().parse::<i32>().unwrap();

            if Self::can_partition(right, target - left_num) {
                return true;
            }
        }

        let num = str_num.iter().collect::<String>().parse::<i32>().unwrap();
        return num == target;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = vec![Input { n: 10 }, Input { n: 37 }];

    for input in inputs {
        let result = Solution::punishment_number(input.n);
        println!("{result:?}");
    }
}
