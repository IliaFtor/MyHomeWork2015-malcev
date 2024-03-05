use std::ops::{Add, AddAssign};

#[derive(Debug)]
struct MyString {
    content: Vec<char>,
}

impl MyString {
    fn new(content: &str) -> MyString {
        MyString { content: content.chars().collect() }
    }

    fn into_string(self) -> String {
        self.content.into_iter().collect()
    }
}

impl Add for MyString {
    type Output = MyString;

    fn add(self, other: MyString) -> MyString {
        let mut result_content = self.content;
        result_content.extend(other.content);
        MyString { content: result_content }
    }
}

impl AddAssign for MyString {
    fn add_assign(&mut self, other: MyString) {
        self.content.extend(other.content);
    }
}

fn main() {
    let str1 = MyString::new("Hello");
    let str2 = MyString::new("world!");

    let result = str1 + str2;
    println!("Результат операции +: {:?}", result.into_string());

    let mut str3 = MyString::new("Rust ");
    let str4 = MyString::new("is ");
    let str5 = MyString::new("awesome!");
    str3 += str4;
    str3 += str5;
    println!("Результат операции +=: {:?}", str3.into_string());
}
