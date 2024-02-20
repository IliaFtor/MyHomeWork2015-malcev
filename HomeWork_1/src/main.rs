use std::io;
fn main() {
    println!("Hello, world!");
    let mut input_x = String::new();
    io::stdin().read_line(&mut input_x).expect("Не удалось прочитать строку");
    let x:i32 = input_x.trim().parse().expect("Введено некорректное число для X");
    let mut input_y = String::new();
    io::stdin().read_line(&mut input_y).expect("Не удалось прочитать строку");
    let y:i32 = input_y.trim().parse().expect("Введено некорректное число для Y");
    println("Задание 4");
    println!("{}",x + y);
    println!("{}",x - y);
    println!("{}",x * y);
    println!("{}",x / y);
    println!("{}",x % y);
    println!("{}",x & y);
    println!("To{}",x ^ y);
    println!("{}",Plus(x,y));

}
fn Plus (x:i32,y:i32)-> String
{
    let result = x + y;
    result.to_string()
}
