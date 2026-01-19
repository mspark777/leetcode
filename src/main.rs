struct Solution;

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        nums.iter().copied().map(Self::encrypt).sum()
    }

    fn encrypt(mut num: i32) -> i32 {
        let mut digits = 0;
        let mut largest = 0;

        while num > 0 {
            digits += 1;
            largest = largest.max(num % 10);
            num /= 10;
        }

        let mut encrypted = 0;
        for _ in 0..digits {
            encrypted *= 10;
            encrypted += largest;
        }

        encrypted
    }
}

struct Input {
    nums: Vec<i32>,
}

fn main() {
    let inputs = [
        Input {
            nums: [1, 2, 3].to_vec(),
        },
        Input {
            nums: [10, 21, 31].to_vec(),
        },
    ];

    for input in inputs {
        let result = Solution::sum_of_encrypted_int(input.nums);
        println!("{:?}", result);
    }
}
