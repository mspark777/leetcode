struct Solution {}

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0; 2 * n - 1];
        let mut is_number_used = vec![false; n + 1];

        Self::find_lexicographically_largest_sequence(0, &mut result, &mut is_number_used, n);

        return result.iter().map(|&i| i as i32).collect();
    }

    fn find_lexicographically_largest_sequence(
        current_index: usize,
        result_sequence: &mut Vec<usize>,
        is_number_used: &mut Vec<bool>,
        target_number: usize,
    ) -> bool {
        if current_index == result_sequence.len() {
            return true;
        }

        if result_sequence[current_index] != 0 {
            return Self::find_lexicographically_largest_sequence(
                current_index + 1,
                result_sequence,
                is_number_used,
                target_number,
            );
        }

        for number_to_place in (1..=target_number).rev() {
            if is_number_used[number_to_place] {
                continue;
            }

            is_number_used[number_to_place] = true;
            result_sequence[current_index] = number_to_place;
            if number_to_place == 1 {
                if Self::find_lexicographically_largest_sequence(
                    current_index + 1,
                    result_sequence,
                    is_number_used,
                    target_number,
                ) {
                    return true;
                }
            } else if ((current_index + number_to_place) < result_sequence.len())
                && (result_sequence[current_index + number_to_place] == 0)
            {
                result_sequence[current_index + number_to_place] = number_to_place;
                if Self::find_lexicographically_largest_sequence(
                    current_index + 1,
                    result_sequence,
                    is_number_used,
                    target_number,
                ) {
                    return true;
                }

                result_sequence[current_index + number_to_place] = 0;
            }

            result_sequence[current_index] = 0;
            is_number_used[number_to_place] = false;
        }

        return false;
    }
}

struct Input {
    n: i32,
}

fn main() {
    let inputs = vec![Input { n: 3 }, Input { n: 5 }];

    for input in inputs {
        let result = Solution::construct_distanced_sequence(input.n);
        println!("{result:?}");
    }
}
