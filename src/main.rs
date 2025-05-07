struct Solution {}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();
        let inf = i32::MAX / 2;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut d = vec![vec![inf; m]; n];
        let mut visit = vec![vec![false; m]; n];
        let n = n as i32;
        let m = m as i32;

        d[0][0] = 0;
        let mut queue =
            std::collections::BinaryHeap::<std::cmp::Reverse<(i32, usize, usize)>>::new();
        queue.push(std::cmp::Reverse((0, 0, 0)));

        while let Some(std::cmp::Reverse((_dir, x, y))) = queue.pop() {
            if visit[x][y] {
                continue;
            }

            visit[x][y] = true;
            for (dx, dy) in dirs.iter().cloned() {
                let nx = (x as i32) + dx;
                let ny = (y as i32) + dy;
                if (nx < 0) || (nx >= n) || (ny < 0) || (ny >= m) {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                let dist = d[x][y].max(move_time[nx][ny]) + 1;
                if d[nx][ny] > dist {
                    d[nx][ny] = dist;
                    queue.push(std::cmp::Reverse((dist, nx, ny)));
                }
            }
        }

        return *d.last().unwrap().last().unwrap();
    }
}

struct Input {
    move_time: Vec<Vec<i32>>,
}

fn main() {
    let inputs = vec![
        Input {
            move_time: vec![vec![0, 4], vec![4, 4]],
        },
        Input {
            move_time: vec![vec![0, 0, 0], vec![0, 0, 0]],
        },
        Input {
            move_time: vec![vec![0, 1], vec![1, 2]],
        },
    ];

    for input in inputs {
        let result = Solution::min_time_to_reach(input.move_time);
        println!("{result:?}");
    }
}
