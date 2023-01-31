struct Solution {}
impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut pairs: Vec<(i32, i32)> = scores.iter().cloned().zip(ages.iter().cloned()).collect();
        pairs.sort_unstable();

        let mut highest_age = 0usize;
        for &age in ages.iter() {
            let a = age as usize;
            highest_age = highest_age.max(a);
        }

        let mut bits = vec![0; highest_age + 1];
        let mut result = i32::min_value();

        for pair in pairs.iter() {
            let score = pair.0;
            let age = pair.1;

            let best = score + Self::query(&bits, age);
            Self::update(&mut bits, age, best);

            result = result.max(best);
        }

        return result;
    }

    fn query(bits: &Vec<i32>, age: i32) -> i32 {
        let mut max_score = i32::min_value();

        let mut i = age;
        while i > 0 {
            max_score = max_score.max(bits[i as usize]);
            i -= i & (-i);
        }

        return max_score;
    }

    fn update(bits: &mut Vec<i32>, age: i32, best: i32) {
        let mut i = age;
        let mut j = i as usize;
        while j < bits.len() {
            bits[j] = bits[j].max(best);
            i += i & (-i);
            j = i as usize;
        }
    }
}

fn main() {
    let inputs = [
        vec![vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]],
        vec![vec![4, 5, 6, 5], vec![2, 1, 2, 1]],
        vec![vec![1, 2, 3, 5], vec![8, 9, 10, 1]],
    ];

    for input in inputs {
        let result = Solution::best_team_score(input[0].clone(), input[1].clone());
        println!("{result}");
    }
}
