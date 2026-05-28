fn rand7() -> i32 {
    7
}

struct Solution;

impl Solution {
    pub fn rand10() -> i32 {
        let mut r = 0;
        while r < 17 {
            r = rand7() * 7 + rand7();
        }

        (r % 10) + 1
    }
}

struct Input {
    query_ip: String,
}

fn main() {
    let inputs = [
        Input {
            query_ip: "172.16.254.1".to_string(),
        },
        Input {
            query_ip: "2001:0db8:85a3:0:0:8A2E:0370:7334".to_string(),
        },
        Input {
            query_ip: "256.256.256.256".to_string(),
        },
    ];

    for input in inputs.into_iter() {
        let result = Solution::valid_ip_address(input.query_ip);
        println!("{:?}", result);
    }
}
