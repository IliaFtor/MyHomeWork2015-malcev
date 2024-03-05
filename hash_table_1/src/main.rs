use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DEFAULT_CAPACITY: usize = 16;

struct Entry<K, V> {
    key: K,
    value: V,
}

struct HashTable<K, V> {
    buffer: Vec<Option<Entry<K, V>>>,
}

impl<K: Eq + Hash, V> HashTable<K, V> {
    fn new() -> Self {
        let mut buffer = Vec::with_capacity(DEFAULT_CAPACITY);
        for _ in 0..DEFAULT_CAPACITY {
            buffer.push(None);
        }
        HashTable { buffer }
    }

    fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % DEFAULT_CAPACITY
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.get_index(&key);
        self.buffer[index] = Some(Entry { key, value });
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_index(key);
        match &self.buffer[index] {
            Some(entry) if entry.key == *key => Some(&entry.value),
            _ => None,
        }
    }
}

fn main() {
    let file = File::open("data.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut hash_table = HashTable::new();

    let start_time = Instant::now();

    for line in reader.lines() {
        if let Ok(value) = line {
            let parts: Vec<&str> = value.split(';').collect();
            if parts.len() == 2 {
                let key = parts[0].trim(); 
                let value = parts[1].trim();
                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher); 
                let hash_value = hasher.finish();

                hash_table.insert(key.to_string(), hash_value);
            }
        }
    }

    let elapsed_time = start_time.elapsed();

    println!("Time taken to convert: {:?}", elapsed_time);

    let seconds = elapsed_time.as_secs_f64();
    println!("Time taken to convert (seconds): {:.6}", seconds);
}
