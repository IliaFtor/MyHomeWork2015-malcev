pub struct Node {
    data: ListData,
    next: Option<Box<Node>>,
}

pub enum ListData {
    Integer(i32),
    Float(f64),
    Str(String),
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    
    pub fn prepend(&mut self, data: ListData) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(), 
        });
        self.head = Some(new_node); 
    }

    pub fn append(&mut self, data: ListData) {
        let new_node = Box::new(Node {
            data,
            next: None,
        });
        let mut current_node = &mut self.head;
        while let Some(ref mut node) = *current_node {
            current_node = &mut node.next;
        }
        *current_node = Some(new_node);
    }
    
    pub fn insert(&mut self, index: usize, data: ListData) {
        if index == 0 {
            self.prepend(data); 
            return;
        }
        let mut current_node = &mut self.head;
        for _ in 0..index {
            let next_node = match current_node {
                Some(node) => &mut node.next,
                None => return, 
            };
            current_node = next_node;
        }
        let new_node = Box::new(Node {
            data,
            next: current_node.take(), 
        });
        *current_node = Some(new_node); 
    }
    pub fn pop_back(&mut self) {
        if let Some(mut current_node) = self.head.take() {
            if current_node.next.is_none() {
                self.head = None;
                return;
            }
            let mut next_node = current_node.next.take();
            while let Some(mut node) = next_node {
                if node.next.is_none() {
                    current_node.next = None;
                    self.head = Some(current_node);
                    return;
                }
                next_node = node.next.take();
                current_node = node;
            }
        }
    }

    
    pub fn pop_front(&mut self) {
        if let Some(mut node) = self.head.take() {
            self.head = node.next.take();
        }
    }

    pub fn remove_at(&mut self, index: usize) {
        if index == 0 {
            self.pop_front();
            return;
        }
        let mut current_node = &mut self.head;
        for _ in 0..index {
            let next_node = match current_node {
                Some(node) => &mut node.next,
                None => return,
            };
            current_node = next_node;
        }
        if let Some(node) = current_node {
            if let Some(next_node) = node.next.take() {
                node.next = next_node.next;
            }
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut ListData> {
        let mut current_node = &mut self.head;
        for _ in 0..index {
            let next_node = match current_node {
                Some(node) => &mut node.next,
                None => return None,
            };
            current_node = next_node;
        }
        current_node.as_mut().map(|node| &mut node.data)
    }

    pub fn get(&self, index: usize) -> Option<&ListData> {
        let mut current_node = &self.head;
        for _ in 0..index {
            let next_node = match current_node {
                Some(node) => &node.next,
                None => return None,
            };
            current_node = next_node;
        }
        current_node.as_ref().map(|node| &node.data)
    }
    pub fn size(&self) -> usize {
        let mut count = 0;
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            count += 1;
            current_node = &node.next;
        }
        count
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn clear(&mut self) {
        self.head = None;
    }

    pub fn front(&self) -> Option<&ListData> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn back(&self) -> Option<&ListData> {
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            if node.next.is_none() {
                return Some(&node.data);
            }
            current_node = &node.next;
        }
        None
    }

    pub fn print(&self) {
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            match &node.data {
                ListData::Integer(i) => print!("{} ", i),
                ListData::Float(f) => print!("{} ", f),
                ListData::Str(s) => print!("{} ", s),
            }
            current_node = &node.next;
        }
        println!();
    }
}
fn main() {
    let mut lst = LinkedList::new();

    println!("Is the list empty? {}", lst.empty());

    for i in 0..5 {
        lst.append(ListData::Str(String::from((b'a' + i) as char)));
    }

    lst.print();

    for i in 0..5 {
        lst.prepend(ListData::Str(String::from((b'z' - i) as char)));
    }

    lst.print();

    for i in 0..lst.size() {
        if let Some(data) = lst.get_mut(i) {
            *data = ListData::Str(String::from((b'a' + i as u8) as char));
        }
    }

    lst.print();

    lst.pop_back();
    lst.pop_front();

    lst.print();

    lst.remove_at(5);
    lst.insert(3, ListData::Str(String::from('o')));

    lst.print();

    lst.clear();

    lst.append(ListData::Str(String::from('q')));
    lst.append(ListData::Str(String::from('w')));

    println!("Size: {}, Is empty? {}", lst.size(), lst.empty());
}


