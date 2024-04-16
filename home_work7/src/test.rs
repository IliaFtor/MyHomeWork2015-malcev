pub struct Test;

impl Test {
    pub fn new() -> Test {
        Test
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("Test object destructed");
    }
}