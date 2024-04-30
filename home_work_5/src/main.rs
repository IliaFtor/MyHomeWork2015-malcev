#![allow(warnings)] 
static mut COUNT_TEST: u32 = 0;

struct Test{
    num: i32,
}
impl Test{
    fn new() -> Self{
        unsafe{
            COUNT_TEST += 1;
        
            println!("Создан, всего: {}", COUNT_TEST);
        }
        Test{num: 1,}
    }
}
impl Drop for Test {
    fn drop(&mut self) {
        unsafe{
        COUNT_TEST -= 1;
        println!("Уничтожен, осталось: {}", COUNT_TEST);
        
        }
    }
}
impl Clone for Test {
    
    fn clone(&self) -> Self {
        unsafe {
            COUNT_TEST += 1;
        }
        Test{ num: self.num }
    }
}
static mut t: Test = Test{num: 2};
fn main(){
    let mut test = Test::new();
    let mut d_test = Box::new(Test::new());
    {
        let mut d_arr_test = vec![];
        for i in 0..10{
            d_arr_test.push(Test::new());
        }
    }
    Foo(test.clone());
    Foo(test.clone());
}
fn Foo(te: Test){
    println!("in foo");
}