mod utils;

use utils::Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut left = 0usize;
        let mut right = letters.len() - 1;
        while left <= right {
            let middle = (left + right) / 2;
            let letter = letters[middle];
            if letter <= target {
                left = middle + 1;
            } else if middle >= 1 {
                right = middle - 1;
            } else {
                left = letters.len();
                break;
            }
        }

        if left < letters.len() {
            return letters[left];
        }

        return letters[0];
    }
}

fn main() {
    let inputs = [
        (vec!['c', 'f', 'j'], 'a'),
        (vec!['c', 'f', 'j'], 'c'),
        (vec!['x', 'x', 'y', 'y'], 'z'),
    ];

    for (letters, target) in inputs {
        let result = Solution::next_greatest_letter(letters, target);
        println!("{result}");
    }
}
