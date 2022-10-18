struct Solution {}
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = vec!["1".to_string()];

        let mut n = n;
        while n > 1 {
            n -= 1;

            let mut temp = Vec::<String>::new();
            let mut count = 1;
            let mut ch = &result[0];
            for c in result.iter().skip(1) {
                if ch == c {
                    count += 1;
                } else {
                    temp.push(format!("{count}"));
                    temp.push(format!("{ch}"));

                    ch = c;
                    count = 1;
                }
            }
            temp.push(format!("{count}"));
            temp.push(format!("{ch}"));
            result = temp;
        }

        return result.join("");
    }
}

fn main() {
    let inputs = [1, 4];

    for input in inputs {
        let result = Solution::count_and_say(input);
        println!("{result}");
    }
}
