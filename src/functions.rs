use std::collections::HashMap;

pub fn studs(){
    //Creating a list of students using Hashmap
    let students = HashMap::from([
        ("Ramos".to_string(), 90),
        ("Stephen".to_string(), 92),
        ("Leonhard".to_string(), 89),
        ("Sanchie".to_string(), 88),
        ("Hannah".to_string(), 91),
        ("Marius".to_string(), 86),
    ]);

    println!("List of Students");
    print_student(&students);

    println!("=====================================");

    search_student(&students,"Alice");

    println!("=====================================");

    println!("Students with grades of 90 or higher");
    honor_students(&students);

    //sample of loop
    let mut _count = 0;
    loop {
        _count += 1;
        if _count == 100{
            break;
        }
    }

    println!("{_count}");
}


//function that prints a list of students
fn print_student(students: &HashMap<String,i32>){
    for (name,grade) in students{
        println!("{}:{}",name,grade);
    }
}

//function that search for a student
fn search_student (students: &HashMap<String,i32>, search_name: &str) {
    if students.get(search_name).is_some(){
        println!("Student {} is found",search_name);
    } else {
        println!("Student is not found");
    }
}

//function that prints a list of students with grades higher than or equal to 90
fn honor_students (students: &HashMap<String,i32>) {
    let mut student_iter = students.iter();

    while let Some((name, &grade)) = student_iter.next() {
        if grade >= 90 {
            println!("{} is an Honor student, with a grade of {}", name, grade);
        } else {
            println!("{} Keep up the good work!", name);
        }
    }
}