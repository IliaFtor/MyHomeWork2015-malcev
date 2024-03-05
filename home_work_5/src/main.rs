struct Test {
    id: i32,
}

impl Test {
    fn new(id: i32) -> Test {
        println!("Test {} created", id);
        Test { id }
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("Test {} destroyed", self.id);
    }
}

static mut COUNT: i32 = 0;

impl Test {
    fn get_count() -> i32 {
        unsafe {
            COUNT
        }
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            COUNT -= 1;
        }
        println!("Test {} destroyed", self.id);
    }
}

fn main() {
    let _ = Test::new(1);
    let _ = Test::new(2);

    println!("Objects count: {}", Test::get_count());
}
