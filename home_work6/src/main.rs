struct MyString<'a>(&'a str);

impl<'a> MyString<'a> {
    fn rfind(&self, pattern: &str, off: usize) -> Option<usize> {
        unsafe {
            let haystack = &self.0[..self.0.len().min(off)];
            let pattern_len = pattern.len();

            if pattern_len == 0 {
                return None;
            }

            for i in (0..=haystack.len() - pattern_len).rev() {
                if haystack.get_unchecked(i..).starts_with(pattern) {
                    return Some(i);
                }
            }

            None
        }
    }
}

fn main() {
    let s = MyString("hello world");
    let pattern = "world";

    // a. Поиск пустой строки
    let empty_pattern = "";
    match s.rfind(empty_pattern, 5) {
        Some(index) => println!("a. Pattern found at index: {}", index),
        None => println!("a. Pattern not found."),
    }

    // b. Два вхождения образца до позиции off
    let double_occurrence = "o";
    match s.rfind(double_occurrence, 5) {
        Some(index) => println!("b. Pattern found at index: {}", index),
        None => println!("b. Pattern not found."),
    }

    // c. Правильная обработка параметра off
    match s.rfind(pattern, 5) {
        Some(index) => println!("c. Pattern found at index: {}", index),
        None => println!("c. Pattern not found."),
    }

    // d. Искомый образец совпадает с началом исходной строки
    match s.rfind("hello", 5) {
        Some(index) => println!("d. Pattern found at index: {}", index),
        None => println!("d. Pattern not found."),
    }
}
