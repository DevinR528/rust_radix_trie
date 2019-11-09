use radix_trie::Trie;

const DATA: &[&str] = &["../data/1984.txt", "../data/sun-rising.txt"];

fn get_text() -> Vec<String> {
    use std::fs::File;
    use std::io::Read;
    const DATA: &[&str] = &["../data/1984.txt", "../data/sun-rising.txt"];
    let mut contents = String::new();
    File::open(&DATA[0])
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
        .split(|c: char| c.is_whitespace())
        .map(|s| s.to_string())
        .collect()
}

fn make_trie(words: &[String]) -> Trie<&str, usize> {
    let mut trie = Trie::new();
    for w in words {
        trie.insert(&w[..], w.len());
    }
    trie
}
// #[flame]
fn trie_insert() {
    let words = get_text();
    make_trie(&words);
}

fn trie_get() {
    let words = get_text();
    let trie = make_trie(&words);

    for w in words.iter() {
        trie.get(&w[..]);
    }
}

fn trie_insert_remove() {
    let words = get_text();
    let mut trie = make_trie(&words);
    for w in &words {
        trie.remove(&w[..]);
    }
}

fn main() {
    for _ in 0..1000 {
        trie_insert_remove()
    }
}
