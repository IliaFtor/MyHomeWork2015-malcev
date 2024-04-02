fn boyer_moore_search(text: &str, pattern: &str) -> Option<usize> {
    let mut count = 0;
    let text_len = text.len();
    let pattern_len = pattern.len();

    if pattern_len == 0 {
        return Some(0);
    }

    let mut bad_char_table = [pattern_len; 256]; 
    for (i, byte) in pattern.bytes().enumerate().take(pattern_len - 1) {
        bad_char_table[byte as usize] = pattern_len - 1 - i;
    }

    let mut offset = 0;
    while offset <= text_len - pattern_len {
        let mut i = pattern_len;
        while i > 0 && pattern.as_bytes()[i - 1] == text.as_bytes()[offset + i - 1] {
            i -= 1;
        }
        if i == 0 {
            println!("кол. итераций: {}",count + pattern_len);
            return Some(offset);
        }
        offset += bad_char_table[text.as_bytes()[offset + i - 1] as usize] as usize;
        count += 1;
    }
    None
}

fn main() {
    let text = "bccbacbbbbabbbbc";
    let pattern = "bbbbc";

    if let Some(index) = boyer_moore_search(text, pattern) {
        println!("Подстрока найдена на позиции: {}", index);
    } else {
        println!("Подстрока не найдена");
    }
}
