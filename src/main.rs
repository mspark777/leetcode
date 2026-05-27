struct Solution;

use std::net::IpAddr;

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        if query_ip.contains("::") {
            return "Neither".to_string();
        }

        match query_ip.parse::<IpAddr>() {
            Ok(IpAddr::V4(_)) => "IPv4".to_string(),
            Ok(IpAddr::V6(_)) => "IPv6".to_string(),
            _ => "Neither".to_string(),
        }
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
