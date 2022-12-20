struct Solution {}
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut seen = vec![false; rooms.len()];
        seen[0] = true;

        let mut stack = vec![0usize];

        while let Some(top) = stack.pop() {
            for &key in rooms[top].iter() {
                let i = key as usize;
                if !seen[i] {
                    seen[i] = true;
                    stack.push(i);
                }
            }
        }

        return !seen.contains(&false);
    }
}

fn main() {
    let inputs = [
        vec![vec![1], vec![2], vec![3], vec![]],
        vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]],
    ];

    for rooms in inputs {
        let result = Solution::can_visit_all_rooms(rooms);
        println!("{result}");
    }
}
