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

    print_student(&students);
    search_student(&students,"Alice");
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

fn search_student (students: &HashMap<String,i32>, search_name: &str){
    // so basically here we would get a reference of the keys inside the hashmap, and ano
    // basta yun i checheck niya kung cinocontain niya yung key na yun.
    // tas pag "students.get(search_name)" lang mag kaka error, kasi yung return ehh "option<&i32" pero yung expected is bool
    // kaya yung is_some() like pag yung option is "(some)", meaning meron siya sa hashmap pag "(none)" namn edi wala.

    if students.get(search_name){
        println!("Student {} is found",search_name);
    } else {
        println!("Student is not found");
    }
}

