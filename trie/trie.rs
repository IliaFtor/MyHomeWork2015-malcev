use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};


#[derive(Default, Debug)]
struct Node{
    character: char,
    value: i32,
    children: Box<HashMap<char, Box<Node>>>,
}
impl Node{
    
}
#[derive(Default, Debug)]
pub struct Trie{
    head: Box<Node>,
}
impl Trie{
    pub fn new() -> Self{
        Trie{
            head: Box::new(Node {
                character: ' ',
                value: 0,
                children: Box::new(HashMap::new()),
            }),
        }
    }
    pub fn insert(&mut self, word: &str){
        let mut curr_node = &mut self.head;

        for c in word.chars() {
            curr_node = curr_node.children.entry(c).or_default();
        }

        curr_node.value += 1;
    }
    pub fn contains(&self, word: &str) -> i32{
        let mut current_node = &self.head;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return 0,
            }
        }

        current_node.value
    }
}
fn main(){
    let mut trie = Trie::new();
    let reader = Box::new(BufReader::new(File::open("data.txt").expect("Cannot open file.txt")));

    let mut stri: String = String::with_capacity(100);
    
    let start = Instant::now();
    //let mut lineN = 0;
    for line in reader.lines() {
        for c in line.unwrap().chars() {
            if c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '/'
            {
                stri.push(c);
            } 
            else if stri.len() > 0
            {
                trie.insert(stri.as_str());
                stri.clear();
            } 
        }
        //lineN += 1;
        //println!("{lineN}");
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("the: {}", trie.contains(&"the"));
    
}