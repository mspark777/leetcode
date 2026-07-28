struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find<'a>(fa: &mut HashMap<&'a String, &'a String>, x: &'a String) -> &'a String {
        if x == fa[x] {
            return x;
        }
        let fd = Solution::find(fa, fa[x]);
        fa.insert(x, fd);
        fa[x]
    }

    pub fn union<'a>(fa: &mut HashMap<&'a String, &'a String>, x: &'a String, y: &'a String) {
        let a = Solution::find(fa, x);
        let b = Solution::find(fa, y);
        fa.insert(b, a);
    }

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut fa = HashMap::new();
        let mut email_name = HashMap::new();
        for acc in accounts.iter() {
            let name = &acc[0];
            for i in 1..acc.len() {
                fa.insert(&acc[i], &acc[i]);
                email_name.entry(&acc[i]).or_insert(name);
            }
        }
        for acc in &accounts {
            for i in 2..acc.len() {
                Solution::union(&mut fa, &acc[1], &acc[i]);
            }
        }
        let mut name_emails = HashMap::new();
        for em in email_name.keys() {
            name_emails
                .entry(Solution::find(&mut fa, *em))
                .or_insert(Vec::new())
                .push(*em);
        }
        for v in name_emails.values_mut() {
            v.sort();
        }

        let mut res = Vec::new();
        for fa_email in name_emails.keys() {
            let mut data = Vec::new();
            data.push(email_name[*fa_email].to_string());
            for t in name_emails.get(fa_email).unwrap().iter() {
                data.push(t.to_string());
            }
            res.push(data);
        }
        res
    }
}

struct Input {
    prices: Vec<i32>,
    fee: i32,
}

fn main() {
    let inputs = [Input {
        prices: [1, 3, 2, 8, 4, 9].to_vec(),
        fee: 2,
    }];

    for input in inputs.into_iter() {
        let result = Solution::max_profit(input.prices, input.fee);
        println!("{:?}", result);
    }
}
