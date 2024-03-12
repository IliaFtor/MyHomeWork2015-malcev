use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::default() }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::default());
        }
        node.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(next_node) = node.children.get(&c) {
                node = next_node;
            } else {
                return false;
            }
        }
        node.is_end_of_word
    }
}

fn main() -> io::Result<()> {
    let mut trie = Trie::new();

    let start = Instant::now();
    let file = File::open("data (2).txt")?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(word) => {
                trie.insert(&word);
            }
            Err(e) => {
                println!("Ошибка при чтении строки: {}", e);
            }
        }
    }
    let duration = start.elapsed();
    let duration = start.elapsed();

    println!("Время чтения и вставки: {:?}", duration);

    let start_search = Instant::now();
    let word_to_find = "wiki";
    if trie.search(word_to_find) {
        println!("Слово '{}' найдено", word_to_find);
    } else {
        println!("Слово '{}' не найдено", word_to_find);
    }
    let duration_search = start_search.elapsed();

    println!("Время выполнения поиска: {} мс", duration_search.as_millis());

    Ok(())
}
