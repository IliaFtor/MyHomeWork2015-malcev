extern crate rand;
extern crate std;

use rand::Rng;
use std::fmt;

fn fill_char_matrix(matrix: &mut Vec<Vec<char>>, percent_pluses: i32) {
    if percent_pluses < 0 || percent_pluses > 100 {
        panic!("Значение процента плюсов должно быть в диапазоне от 0 до 100.");
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut rng = rand::thread_rng();
    let num_pluses = (rows * cols * percent_pluses) / 100;

    for i in 0..rows {
        for j in 0..cols {
            matrix[i][j] = ' ';
        }
    }

    for _ in 0..num_pluses {
        let random_row = rng.gen_range(0..rows);
        let random_col = rng.gen_range(0..cols);

        if matrix[random_row][random_col] != '+' {
            matrix[random_row][random_col] = '+';
        } else {
            continue;
        }
    }
}

fn fill(x: usize, y: usize, a: &mut Vec<Vec<char>>) {
    let rows = a.len();
    let cols = a[0].len();

    if x < rows && y < cols && a[x][y] == ' ' {
        a[x][y] = '+';

        if x > 0 {
            fill(x - 1, y, a);
        }
        if y > 0 {
            fill(x, y - 1, a);
        }
        if x < rows - 1 {
            fill(x + 1, y, a);
        }
        if y < cols - 1 {
            fill(x, y + 1, a);
        }
    }
}

fn main() {
    let rows = 20;
    let cols = 20;

    let mut char_matrix = vec![vec![' '; cols]; rows];

    fill_char_matrix(&mut char_matrix, 40);

    let start_x = 2;
    let start_y = 2;

    fill(start_x, start_y, &mut char_matrix);

    for i in 0..rows {
        for j in 0..cols {
            print!("{} ", char_matrix[i][j]);
        }
        println!();
    }
}
/*
[dependencies]
rand = "0.5.5"
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);

}*/