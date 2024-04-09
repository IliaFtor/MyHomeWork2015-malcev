use std::fmt;

type StudentData = (String, String, bool, usize, (f64, usize));

fn print_student(student: &StudentData) {
    let (name, surname, gender, zachete_id, (mean, number)) = student;
    println!("{:<10} {:<10} {} {} {:.2} {}", name, surname, gender, zachete_id, mean, number);
}

fn adjacent_find(group: &Vec<StudentData>) -> Option<(&StudentData, &StudentData)> {
    group.windows(2)
        .find(|pair| {
            let (_, _, _, _, (_, num_disc1)) = &pair[0];
            let (_, _, _, _, (_, num_disc2)) = &pair[1];
            let (name1, _, _, _, _) = &pair[0];
            let (name2, _, _, _, _) = &pair[1];
            num_disc1 == num_disc2 && name1.chars().last() == name2.chars().last()
        })
        .map(|pair| (&pair[0], &pair[1]))
}

fn find_if(group: &Vec<StudentData>) -> Option<&StudentData> {
    group.iter().find(|(_, _, _, _, (mean, _))| *mean < 3.0)
}


fn main() {
    let group: Vec<StudentData> = vec![
        ("Borikin".to_string(), "Bogdan".to_string(), true, 8434562, (4.92, 12)),
        ("Borikin".to_string(), "Anderin".to_string(), true, 8434562, (4.92, 12)),
        ("Kirlenko".to_string(), "Maksim".to_string(), false, 6349523, (4.81, 13)),
        ("Bog".to_string(), "Bogdan".to_string(), true, 8434562, (4.92, 12)),
        ("Bonapart".to_string(), "Egor".to_string(), true, 2456246, (4.55, 8)),
        ("Macev".to_string(), "ilia".to_string(), false, 4567890, (3.75, 7)),
        ("Geiii".to_string(), "Erdem".to_string(), true, 9876543, (2.10, 9)),
    ];
    println!("Данные о студентах:");
    for student in &group {
        print_student(student);
    }
    println!();

    if let Some((student1, student2)) = adjacent_find(&group) {
        println!("Найдены студенты с одинаковым числом сданных дисциплин и совпадающими последними символами в имени:");
        println!("Первый студент:");
        print_student(student1);
        println!("Второй студент:");
        print_student(student2);
    } else {
        println!("Такие студенты не найдены.");
    }
    println!();

    if let Some(student) = find_if(&group) {
        println!("Найден студент со средней оценкой меньше 3:");
        print_student(student);
    } else {
        println!("Студентов со средней оценкой меньше 3 не найдено.");
    }
}
