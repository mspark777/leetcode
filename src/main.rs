struct Solution {}
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut result = Vec::<String>::new();

        for h in 0..12 {
            for m in 0..60 {
                let num: i32 = (h << 6) | m;
                let ones = num.count_ones();
                if ones == turned_on {
                    if m >= 10 {
                        let time = format!("{h}:{m}");
                        result.push(time);
                    } else {
                        let time = format!("{h}:0{m}");
                        result.push(time);
                    }
                }
            }
        }

        return result;
    }
}

fn main() {
    let inputs = [1, 9];

    for turned_on in inputs {
        let result = Solution::read_binary_watch(turned_on);
        println!("{result:?}");
    }
}
