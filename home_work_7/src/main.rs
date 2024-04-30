use std::collections::LinkedList;
use std::time::Instant;

fn main() {
    // Создаем связный список для хранения целых чисел
    let mut my_list: LinkedList<i32> = LinkedList::new();

    // Число элементов в контейнере
    let n = 200;
    // Количество операций вставки и удаления
    let m = 2000;

    // Заполнение списка числами от 0 до n
    for i in 0..n {
        my_list.push_back(i);
    }

    // Переменная k
    let mut k = 0;

    // Добавление и удаление элемента m раз
    for i in 0..m {
        // Добавление элемента в начало списка
        my_list.push_front(i);
        // Удаление элемента из начала списка
        my_list.pop_front();
        // Увеличение k
        k += 1;
    }

    // Вывод значения k
    println!("Value of k: {}", k);

    // Измерение времени выполнения
    let start = Instant::now();

    // Добавление и удаление элемента m раз
    for i in 0..m {
        // Добавление элемента в начало списка
        my_list.push_front(i);
        // Удаление элемента из начала списка
        my_list.pop_front();
    }

    let duration = start.elapsed();

    // Вывод времени выполнения
    println!("Time elapsed: {:?}", duration);
        // Создаем связный список для хранения целых чисел
        let mut my_list: LinkedList<i32> = LinkedList::new();

        // Число элементов в контейнере
        let n = 200;
        // Количество операций вставки и удаления
        let m = 2000;
    
        // Заполнение списка числами от 0 до n
        for i in 0..n {
            my_list.push_back(i);
        }
    
        // Переменная k
        let mut k = 0;
    
        // Добавление и удаление элемента m раз
        for i in 0..m {
            // Добавление элемента в начало списка
            my_list.push_front(i);
            // Удаление элемента из начала списка
            my_list.pop_front();
            // Увеличение k
            k += 1;
        }
    
        // Вывод значения k
        println!("Value of k: {}", k);
    
        // Измерение времени выполнения
        let start = Instant::now();
    
        // Добавление и удаление элемента m раз
        for i in 0..m {
            // Добавление элемента в начало списка
            my_list.push_front(i);
            // Удаление элемента из начала списка
            my_list.pop_front();
        }
    
        let duration = start.elapsed();
    
        // Вывод времени выполнения
        println!("Time elapsed: {:?}", duration);
}
