struct Solution {}
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort_unstable();

        let search_word = search_word.as_bytes();
        let mut results: Vec<Vec<String>> = vec![Vec::new(); search_word.len()];

        for product in &products {
            let bytes = product.as_bytes();
            for i in 0..bytes.len().min(search_word.len()) {
                if search_word[i] != bytes[i] {
                    break;
                } else if results[i].len() < 3 {
                    results[i].push(product.clone());
                }
            }
        }

        results
    }
}

fn main() {
    let inputs = [
        (
            vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"],
            "mouse",
        ),
        (vec!["havana"], "havana"),
        (vec!["havana"], "tatiana"),
        (vec!["bags", "baggage", "banner", "box", "cloths"], "bags"),
    ];

    for input in inputs {
        let products = input.0.iter().map(|s| String::from(*s)).collect();
        let search_word = String::from(input.1);
        let result = Solution::suggested_products(products, search_word);
        println!("{result:?}");
    }
}
