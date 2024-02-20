use std::mem;

fn main() {
    //println!("{}", task_1());

    let x: [i32; 5] = [1, 2, 3, 4, 5];
    for element in x.iter() {
        println!("{}", element);
    }

    let str = b"Hello!";
    println!("{}", String::from_utf8_lossy(str));

    println!("Длина строки: {}", len(str));
    println!("Размер массива: {}", mem::size_of_val(str));

    let mut t = [0u8; 32];
    string_copy(&mut t, str);

    println!("Скопированная строка: {}", String::from_utf8_lossy(&t));

    let mut another_t = [0u8; 32];
    string_copy(&mut another_t, str);
    println!("Скопированная строка с помощью функции: {}", String::from_utf8_lossy(&another_t));
}

pub fn len(s: &[u8]) -> usize {
    s.len()
}

fn string_copy(d: &mut [u8], s: &[u8]) {
    for (dst, src) in d.iter_mut().zip(s.iter()) {
        *dst = *src;
    }
}

pub fn compare(s: &[u8], t: &[u8]) -> i32 {
    s.cmp(t) as i32
}

fn task_1() -> String {
    let mut p: *mut u8 = 0x123 as *mut u8;
    p = unsafe {
        p.offset(-1)
    };
    let value = unsafe {
        *p
    };
    format!("Значение по адресу: {}", value)
}
