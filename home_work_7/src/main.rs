use std::collections::LinkedList;
use std::time::Instant;

fn main() {
    for n in (100..=2000).step_by(100) {
        let mut my_list: LinkedList<i32> = LinkedList::new();
        let m = 2000;

        for i in 0..n {
            my_list.push_back(i);
        }

        let mut k = 0;

        for i in 0..m {
            my_list.push_front(i);
            my_list.pop_front();
            k += 1;
        }

        println!("Value of k for n = {}: {:?}", n, k);

        let start = Instant::now();

        for i in 0..m {
            my_list.push_front(i);
            my_list.pop_front();
        }

        let duration = start.elapsed();

        println!("Time elapsed for n = {:?}: {:?}", n, duration);
    }
}
