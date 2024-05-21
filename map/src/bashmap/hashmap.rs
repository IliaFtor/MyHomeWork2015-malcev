mod bash;
mod equ;


use std::ops::AddAssign;
use bash::Hash;
use equ::Equ;

#[derive(Clone)]
struct Entity<K: Clone, V:Copy>{
    key: K,
    hash: usize,
    value: V,
    next: i32,
}

pub struct HashMap<K : Hash+Clone, V : Copy>{
    buckets: Vec<i32>,
    entities: Vec<Entity<K, V>>,
    count: usize,
}
impl<K : Hash+Equ+Clone, V : AddAssign+Copy> HashMap<K, V>{
    pub fn new() -> Self{
        HashMap{
            buckets: vec![-1; 7],
            entities: Vec::with_capacity(7),
            count: 0,
        }
    }
    pub fn add(&mut self, key: K, val: V){
        let hash = key.hash();
        let mut bucket = hash % self.buckets.len();

        let mut i = self.buckets[bucket];
        while i>=0{
            if self.entities[i as usize].hash == hash && key.equals(&self.entities[i as usize].key){
                self.entities[i as usize].value += val;
                return;
            }
            i = self.entities[i as usize].next;
        }
        let index;

        if self.count == self.entities.capacity(){
            self.resize();
            bucket = hash % self.buckets.len();
        }

        index = self.count;
        self.count+=1;

        self.entities.push(Entity{
            key,
            hash,
            value: val,
            next: self.buckets[bucket],
        });
        self.buckets[bucket] = index as i32;
    }
    fn resize(&mut self){
        let new_size = self.count * 2;
        let mut new_buckets = Vec::with_capacity(new_size);
        for _i in 0..new_size{ new_buckets.push(-1); }
        let mut new_entities = Vec::with_capacity(new_size);
        for e in 0..self.entities.len(){
            new_entities.push(self.entities[e].clone());
        }
        for i in 0..self.count
        {
            if new_entities[i].hash >= 0
            {
                let bucket = new_entities[i].hash % new_size;
                new_entities[i].next = new_buckets[bucket];
                new_buckets[bucket] = i as i32;
            }
        }
        self.buckets = new_buckets;
        self.entities = new_entities;
    }
    pub fn get(&self, key: &K) -> Option<V>{
        let hash = key.hash();
        let bucket = hash % self.buckets.len();

        if self.buckets[bucket] == -1 {
            return None;
        }
        let mut i = self.buckets[bucket];
        while i>=0{
            if self.entities[i as usize].hash == hash && key.equals(&self.entities[i as usize].key){
                return Some(self.entities[i as usize].value);
            }
            i = self.entities[i as usize].next;
        }
        None
    }
}

