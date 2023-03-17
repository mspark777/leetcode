mod utils;
use utils::Trie;

fn main() {
    let apple = "apple".to_string();
    let app = "app".to_string();

    let mut trie = Trie::new();
    trie.insert(apple.clone());
    println!("{}", trie.search(apple.clone()));
    println!("{}", trie.search(app.clone()));
    println!("{}", trie.starts_with(app.clone()));
    trie.insert(app.clone());
    println!("{}", trie.search(app.clone()));
}
