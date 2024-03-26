use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Entry {
    hash_code: usize,
    next: Option<usize>,
    key: String,
    value: isize,
}

#[derive(Debug)]
pub struct HashTable {
    buckets: Vec<Option<usize>>,
    entries: Vec<Entry>,
    count: usize,
}

impl HashTable {
    pub fn new() -> Self {
        HashTable {
            buckets: vec![None; 16], 
            entries: Vec::with_capacity(16),
            count: 0,
        }
    }

    fn add(&mut self, key: String, value: isize) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_code = hasher.finish() as usize;
        let target_bucket = hash_code % self.buckets.len();

        let entry_index = self.entries.len();
        self.entries.push(Entry {
            hash_code,
            next: self.buckets[target_bucket],
            key: key.clone(),
            value,
        });

        self.buckets[target_bucket] = Some(entry_index);
        self.count += 1;
    }

    fn count(&self, key: &str) -> Option<usize> {
        let mut count = 0;
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_code = hasher.finish() as usize;
        let target_bucket = hash_code % self.buckets.len();

        let mut entry_index = match self.buckets[target_bucket] {
            Some(idx) => idx,
            None => return Some(count),
        };

        loop {
            let entry = &self.entries[entry_index];
            if entry.hash_code == hash_code && entry.key == key {
                count += 1;
            }
            match entry.next {
                Some(next_index) => entry_index = next_index,
                None => break,
            }
        }

        Some(count)
    }
}

fn main() {
    let file = File::open("data (2).txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut hash_table = HashTable::new();
    let start_time_convert = Instant::now();
    for line in reader.lines() {
        if let Ok(value) = line {
            let parts: Vec<&str> = value.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                let key = parts[0].to_string();
                let value = parts[1].parse::<isize>().unwrap_or(0);
                hash_table.add(key, value);
            }
        }
    }
    let convert_time = start_time_convert.elapsed();
    println!("Time taken: {:?}", convert_time);

    let start_time = Instant::now();
    let key_to_search = "Type";
    if let Some(value) = hash_table.count(key_to_search) {
        println!("Value for key {}: {}", key_to_search, value);
    } else {
        println!("Key {} not found", key_to_search);
    }

    let search_seconds = start_time.elapsed().as_secs_f64();
    println!("Time Search (seconds): {:.6}s", search_seconds);
    
}
