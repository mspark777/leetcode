struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total <= 0 {
            return true;
        }

        if (max_choosable_integer * (max_choosable_integer + 1)) / 2 < desired_total {
            return false;
        }

        Self::is_next_player_winning(0, max_choosable_integer, desired_total, &mut HashMap::new())
    }

    fn is_next_player_winning(
        mut selected: i32,
        max_choose: i32,
        total: i32,
        memo: &mut HashMap<i32, bool>,
    ) -> bool {
        if total <= 0 {
            return false;
        }

        let state = selected;
        if let Some(&value) = memo.get(&state) {
            return value;
        }

        let mut curr_player_wins = false;
        for i in 0..max_choose {
            if (selected & (1 << i)) == 0 {
                selected |= 1 << i;
                if !Self::is_next_player_winning(selected, max_choose, total - (i + 1), memo) {
                    curr_player_wins = true;
                    break;
                }
                selected &= !(1 << i);
            }
        }

        memo.insert(state, curr_player_wins);
        curr_player_wins
    }
}

struct Input {
    max_choosable_integer: i32,
    desired_total: i32,
}

fn main() {
    let inputs = [
        Input {
            max_choosable_integer: 10,
            desired_total: 11,
        },
        Input {
            max_choosable_integer: 10,
            desired_total: 0,
        },
        Input {
            max_choosable_integer: 10,
            desired_total: 1,
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::can_i_win(input.max_choosable_integer, input.desired_total);
        println!("{:?}", result);
    }
}
