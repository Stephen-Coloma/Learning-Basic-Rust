use std::collections::HashMap;

fn main() {
    // the '::from' automatically converts the values inside as an element of Hashmap,
    // rather than manually inserting them
    // like "students.insert(values);"
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
}

//function that prints a list of students
// for (value a) in variables{
// ----- other lines of codes ---
//}
fn print_student(students: &HashMap<String,i32>){
    for (name,grade) in students{
        println!("{}:{}",name,grade);
    }
}

//function that search for a student
// the .is_some() basically returns a boolean value, it is either (none) or (some)
// the .get(), is commonly used in hashmaps to check a key if it contains that value or not

fn search_student (students: &HashMap<String,i32>, search_name: &str) {
    // so basically here we would get a reference of the keys inside the hashmap, and ano
    // basta yun i checheck niya kung cinocontain niya yung key na yun.
    // tas pag "students.get(search_name)" lang mag kaka error, kasi yung return ehh "option<&i32" pero yung expected is bool
    // kaya yung is_some() like pag yung option is "(some)", meaning meron siya sa hashmap pag "(none)" namn edi wala.

    if students.get(search_name).is_some(){
        println!("Student {} is found",search_name);
    } else {
        println!("Student is not found");
    }
}

//Printing students with grades higher than or equal to 90
fn honor_students (students: &HashMap<String,i32>){
    // making use of the .iter() provides a way to access the values, without copying the whole hashmap
    let mut student_iter = students.iter();

    // "while let" is used for matching values
    // so basically here, the Some(value) holds the value to check so if the student_iter.next returns "some"
    // meaning there are still values to check on the iteration, but when the student_iter.next returns "none"
    // the "while let" will stop executing
    while let Some((name,&grade)) = student_iter.next() {
        if grade >= 90{
            println!("{} is an Honor student, with a grade of {}",name,grade);
        } else {
            println!("{} Keep up the good work!",name);
        }
    }
}
