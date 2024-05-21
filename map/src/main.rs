mod bashmap;

use bashmap::hashmap::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::fs::File;
use std::time::Instant;

fn main() {

    let mut h: HashMap<String, i32> = HashMap::new();
    let reader = BufReader::new(File::open("C:/Mine/work/programming/projects/Rust/hashmap2/src/engwiki_ascii.txt").expect("Cannot open file.txt"));

    let mut stri: String = String::with_capacity(100);

    let start = Instant::now();
    // let ws = "wiki".to_string();
    //let mut lineN = 0;
    for c in reader.bytes(){
            let c = c.unwrap();
            if (c >= 97 && c <= 122) || (c >= 65 && c <= 90) || c == 39
            {
                stri.push(c as char);
            }
            else if stri.len() > 0
            {
                h.add(String::from(stri.clone()), 1);

                // if stri == ws{
                //     println!("{}", h.get(&String::from("wiki")).unwrap());
                // }
                stri.clear();
            }

        //lineN += 1;
        //println!("{lineN}");
    }
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("N of \"wiki\":{}", h.get(&String::from("wiki")).unwrap());
    // let mut line = String::new();
    // let b1 = std::io::stdin().read_line(&mut line).unwrap();
}
