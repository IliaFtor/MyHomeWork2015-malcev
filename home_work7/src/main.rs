//mod test; 
//use test::Test;

use std::rc::{Rc, Weak};
struct Test;

impl Test {
    fn new() -> Test {
        Test
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("Test object destructed");
    }
}
fn main() {
    {/// первая часть
    let mut my_vec = vec![7, 2, 5, 1, 9];
    
        let mut my_set = vec![7, 2, 5, 1, 9].into_iter().collect::<std::collections::HashSet<_>>();
    
        println!("Исходный вектор: {:?}", my_vec);
        println!("Исходное множество: {:?}", my_set);
    
        my_vec.sort();
    
        let mut sorted_set: Vec<_> = my_set.into_iter().collect();
        sorted_set.sort();
    
        println!("Отсортированный вектор: {:?}", my_vec);
        println!("Отсортированное множество: {:?}", sorted_set);
    }
///2 часть
    {
    
        // Внутренний блок для демонстрации времени жизни объектов
        {
        println!("Inner block");

            //(умный указатель)
            let sp = Rc::new(Test::new());
            println!("Strong reference count: {}", Rc::strong_count(&sp));

            //(тупой указатель)
            let wp = Rc::downgrade(&sp);
            println!("Strong reference count after weak pointer creation: {}", Rc::strong_count(&sp));

            // инстетут
            if let Some(p) = wp.upgrade() {
                println!("Object is alive. Strong reference count: {}", Rc::strong_count(&p));
            } else {
                println!("No owning object");
            }

            // дипломная работа
            if let Some(p) = wp.upgrade() {
                println!("Object is still alive. Strong reference count: {}", Rc::strong_count(&p));
            } else {
                println!("No owning object anymore");
            }
        }  
    }
}

    