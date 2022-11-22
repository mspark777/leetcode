struct Solution {}
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut memos = vec![i32::max_value(); n + 1];

        memos[0] = 0;
        let mut cur = 1usize;
        let mut squire = 1usize;

        while squire <= n {
            for i in squire..=n {
                memos[i] = memos[i].min(memos[i - squire] + 1);
            }

            cur += 1;
            squire = cur * cur;
        }

        return memos[n];
    }
}

fn main() {
    let inputs = [12, 13];

    for n in inputs {
        let result = Solution::num_squares(n);
        println!("{result}");
    }
}
